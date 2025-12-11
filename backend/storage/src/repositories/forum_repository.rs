use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        forum::{
            EditedForumPost, ForumPost, ForumPostAndThreadName, ForumPostHierarchy,
            ForumSearchQuery, ForumSearchResult, ForumThread, ForumThreadEnriched,
            GetForumThreadPostsQuery, UserCreatedForumPost, UserCreatedForumThread,
        },
        user::UserLiteAvatar,
    },
};
use arcadia_common::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, PgPool};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_forum_post(
        &self,
        forum_post: &UserCreatedForumPost,
        current_user_id: i32,
    ) -> Result<ForumPost> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let created_forum_post = sqlx::query_as!(
            ForumPost,
            r#"
                INSERT INTO forum_posts (content, created_by_id, forum_thread_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            forum_post.content,
            current_user_id,
            forum_post.forum_thread_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumPost)?;

        sqlx::query!(
            r#"
            UPDATE forum_threads
            SET posts_amount = posts_amount + 1
            WHERE id = $1;
            "#,
            forum_post.forum_thread_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumPost)?;

        sqlx::query!(
            r#"
            UPDATE forum_sub_categories
            SET posts_amount = posts_amount + 1
            WHERE id = (SELECT forum_sub_category_id FROM forum_threads WHERE id = $1);
            "#,
            forum_post.forum_thread_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumPost)?;

        Self::notify_users_forum_thread_posts(
            &mut tx,
            forum_post.forum_thread_id,
            created_forum_post.id,
            current_user_id,
        )
        .await?;

        tx.commit().await?;

        Ok(created_forum_post)
    }

    pub async fn find_forum_post(&self, forum_post_id: i64) -> Result<ForumPost> {
        let forum_post = sqlx::query_as!(
            ForumPost,
            r#"
                SELECT * FROM forum_posts WHERE id = $1
            "#,
            forum_post_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumPost)?;

        Ok(forum_post)
    }

    pub async fn update_forum_post(&self, edited_post: &EditedForumPost) -> Result<ForumPost> {
        let updated_post = sqlx::query_as!(
            ForumPost,
            r#"
                UPDATE forum_posts
                SET content = $1, sticky = $2, locked = $3
                WHERE id = $4
                RETURNING *
            "#,
            edited_post.content,
            edited_post.sticky,
            edited_post.locked,
            edited_post.id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateForumPost)?;

        Ok(updated_post)
    }

    pub async fn create_forum_thread(
        &self,
        forum_thread: &mut UserCreatedForumThread,
        current_user_id: i32,
    ) -> Result<ForumThread> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let created_forum_thread = sqlx::query_as!(
            ForumThread,
            r#"
                INSERT INTO forum_threads (name, created_by_id, forum_sub_category_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            forum_thread.name,
            current_user_id,
            forum_thread.forum_sub_category_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumThread)?;

        forum_thread.first_post.forum_thread_id = created_forum_thread.id;

        sqlx::query!(
            r#"
            UPDATE forum_sub_categories
            SET threads_amount = threads_amount + 1
            WHERE id = $1;
            "#,
            forum_thread.forum_sub_category_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumPost)?;

        tx.commit().await?;

        // TODO: include this in the transaction
        self.create_forum_post(&forum_thread.first_post, current_user_id)
            .await?;

        Ok(created_forum_thread)
    }

    pub async fn find_forum_cateogries_hierarchy(&self) -> Result<Value> {
        let forum_overview = sqlx::query!(
            r#"
            SELECT
                json_build_object(
                    'forum_categories', json_agg(
                        json_build_object(
                            'id', fc.id,
                            'name', fc.name,
                            'sub_categories', (
                                SELECT
                                    json_agg(
                                        json_build_object(
                                            'id', fsc.id,
                                            'name', fsc.name,
                                            'threads_amount', fsc.threads_amount,
                                            'posts_amount', fsc.posts_amount,
                                            'forbidden_classes', '[]'::jsonb,
                                            'latest_post_in_thread', CASE
                                                WHEN ft.id IS NOT NULL THEN json_build_object(
                                                    'id', ft.id,
                                                    'name', ft.name,
                                                    'created_at', ft.latest_post_created_at,
                                                    'created_by', json_build_object( -- Changed to a JSON object for user details
                                                        'id', ft.latest_post_created_by_id,
                                                        'username', ft.latest_post_created_by_username
                                                    ),
                                                    'posts_amount', ft.posts_amount
                                                )
                                                ELSE NULL
                                            END
                                        ) ORDER BY fsc.name
                                    )
                                FROM
                                    forum_sub_categories fsc
                                LEFT JOIN LATERAL (
                                    SELECT
                                        ft_with_latest_post.id,
                                        ft_with_latest_post.name,
                                        ft_with_latest_post.posts_amount,
                                        fp_latest.created_at AS latest_post_created_at,
                                        fp_latest.created_by_id AS latest_post_created_by_id,
                                        u.username AS latest_post_created_by_username -- Joined to get the username
                                    FROM
                                        forum_posts fp_latest
                                    JOIN
                                        forum_threads ft_with_latest_post ON fp_latest.forum_thread_id = ft_with_latest_post.id
                                    JOIN
                                        users u ON fp_latest.created_by_id = u.id -- Joined with the users table
                                    WHERE
                                        ft_with_latest_post.forum_sub_category_id = fsc.id
                                    ORDER BY
                                        fp_latest.created_at DESC
                                    LIMIT 1
                                ) AS ft ON TRUE
                                WHERE
                                    fsc.forum_category_id = fc.id
                            )
                        ) ORDER BY fc.id
                    )
                ) AS forum_overview
            FROM
                forum_categories fc;
            "#
        )
        .fetch_one(self.borrow())
        .await
        .expect("error getting forums");

        Ok(forum_overview
            .forum_overview
            .unwrap()
            .get("forum_categories")
            .unwrap_or(&json!([]))
            .to_owned())
    }

    pub async fn find_forum_sub_category_threads(
        &self,
        forum_sub_category_id: i32,
    ) -> Result<Value> {
        let forum_sub_category = sqlx::query!(
            r#"
            SELECT
                json_strip_nulls(
                    json_build_object(
                        'id', fsc.id,
                        'name', fsc.name,
                        'threads_amount', fsc.threads_amount,
                        'posts_amount', fsc.posts_amount,
                        'forbidden_classes', fsc.forbidden_classes,
                        'category', json_build_object(
                            'id', fc.id,
                            'name', fc.name
                        ),
                        'threads', (
                            SELECT
                                COALESCE(
                                    json_agg(
                                        json_build_object(
                                            'id', ft.id,
                                            'name', ft.name,
                                            'created_at', ft.created_at,
                                            'posts_amount', ft.posts_amount,
                                            'latest_post', CASE
                                                WHEN fp_latest.id IS NOT NULL THEN json_build_object(
                                                    'id', fp_latest.id,
                                                    'created_at', fp_latest.created_at,
                                                    'created_by', json_build_object(
                                                        'id', u_post.id,
                                                        'username', u_post.username
                                                    )
                                                )
                                                ELSE NULL
                                            END
                                        ) ORDER BY ft.created_at DESC
                                    ),
                                    '[]'::json
                                )
                            FROM
                                forum_threads ft
                            LEFT JOIN LATERAL (
                                SELECT
                                    fp.id,
                                    fp.created_at,
                                    fp.created_by_id
                                FROM
                                    forum_posts fp
                                WHERE
                                    fp.forum_thread_id = ft.id
                                ORDER BY
                                    fp.created_at DESC
                                LIMIT 1
                            ) AS fp_latest ON TRUE
                            LEFT JOIN
                                users u_post ON fp_latest.created_by_id = u_post.id
                            WHERE
                                ft.forum_sub_category_id = fsc.id
                        )
                    )
                ) AS result_json
            FROM
                forum_sub_categories fsc
            JOIN
                forum_categories fc ON fsc.forum_category_id = fc.id
            WHERE
                fsc.id = $1
            GROUP BY
                fsc.id, fc.id;
            "#,
            forum_sub_category_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumSubCategory)?;

        //TODO: unwrap can fail return Error::CouldNotFindForumSubCategory
        Ok(forum_sub_category.result_json.unwrap())
    }

    pub async fn find_forum_thread(
        &self,
        forum_thread_id: i64,
        user_id: i32,
    ) -> Result<ForumThreadEnriched> {
        let forum_thread = sqlx::query_as!(
            ForumThreadEnriched,
            r#"
            SELECT
                ft.id,
                ft.forum_sub_category_id,
                ft.name,
                ft.created_at,
                ft.created_by_id,
                ft.posts_amount,
                ft.sticky,
                ft.locked,
                fsc.name AS forum_sub_category_name,
                fc.name AS forum_category_name,
                fc.id AS forum_category_id,
                (sft.id IS NOT NULL) AS "is_subscribed!"
            FROM
                forum_threads AS ft
            JOIN
                forum_sub_categories AS fsc ON ft.forum_sub_category_id = fsc.id
            JOIN
                forum_categories AS fc ON fsc.forum_category_id = fc.id
            LEFT JOIN
                subscriptions_forum_thread_posts AS sft
                ON sft.forum_thread_id = ft.id AND sft.user_id = $2
            WHERE
                ft.id = $1;
            "#,
            forum_thread_id,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThread)?;

        if forum_thread.is_subscribed {
            Self::mark_notification_forum_thread_post_as_read(self, forum_thread_id, user_id)
                .await?;
        }

        Ok(forum_thread)
    }

    pub async fn find_forum_thread_posts(
        &self,
        form: GetForumThreadPostsQuery,
    ) -> Result<PaginatedResults<ForumPostHierarchy>> {
        let page_size = form.page_size as i64;
        let mut current_page = form.page.unwrap_or(1);

        let offset = if let Some(post_id) = form.post_id {
            let position = sqlx::query_scalar!(
                r#"
                SELECT COUNT(*)::BIGINT FROM forum_posts
                WHERE forum_thread_id = $1 AND id < $2
                "#,
                form.thread_id,
                post_id
            )
            .fetch_one(self.borrow())
            .await?
            .unwrap_or(0);

            // i64 ceil division is unstable as of now
            current_page = ((position + 1) as u64).div_ceil(form.page_size as u64) as u32;
            ((position / page_size) * page_size) as i64
        } else {
            ((form.page.unwrap_or(1) - 1) as i64) * page_size
        };

        #[derive(Debug, FromRow)]
        struct DBImportForumPost {
            id: i64,
            content: String,
            created_at: DateTime<Utc>,
            updated_at: DateTime<Utc>,
            sticky: bool,
            locked: bool,
            forum_thread_id: i64,
            created_by_user_id: i32,
            created_by_user_username: String,
            created_by_user_avatar: Option<String>,
            created_by_user_banned: bool,
            created_by_user_warned: bool,
        }

        let posts = sqlx::query_as!(
            DBImportForumPost,
            r#"
            SELECT
                fp.id,
                fp.content,
                fp.created_at,
                fp.updated_at,
                fp.sticky,
                fp.locked,
                fp.forum_thread_id,
                u.id AS created_by_user_id,
                u.username AS created_by_user_username,
                u.avatar AS created_by_user_avatar,
                u.banned AS created_by_user_banned,
                u.warned AS created_by_user_warned
            FROM forum_posts fp
            JOIN users u ON fp.created_by_id = u.id
            WHERE fp.forum_thread_id = $1
            ORDER BY fp.created_at ASC
            OFFSET $2
            LIMIT $3
            "#,
            form.thread_id,
            offset,
            page_size
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThread)?;

        let total_forum_posts_in_thread = sqlx::query_scalar!(
            r#"SELECT COUNT(id) FROM forum_posts WHERE forum_thread_id = $1"#,
            form.thread_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThread)?
        .unwrap_or(0);

        let forum_posts: Vec<ForumPostHierarchy> = posts
            .into_iter()
            .map(|r| ForumPostHierarchy {
                id: r.id,
                content: r.content,
                created_at: r.created_at,
                updated_at: r.updated_at,
                sticky: r.sticky,
                locked: r.locked,
                forum_thread_id: r.forum_thread_id,
                created_by: UserLiteAvatar {
                    id: r.created_by_user_id,
                    username: r.created_by_user_username,
                    avatar: r.created_by_user_avatar,
                    banned: r.created_by_user_banned,
                    warned: r.created_by_user_warned,
                },
            })
            .collect();

        let paginated_results = PaginatedResults {
            results: forum_posts,
            page: current_page,
            page_size: form.page_size,
            total_items: total_forum_posts_in_thread,
        };

        Ok(paginated_results)
    }

    pub async fn find_first_thread_posts_in_sub_category(
        &self,
        forum_sub_category_id: i32,
        limit: u32,
    ) -> Result<Vec<ForumPostAndThreadName>> {
        sqlx::query_as!(
            ForumPostAndThreadName,
            r#"
            SELECT DISTINCT ON (ft.id)
                fp.id,
                fp.forum_thread_id,
                fp.created_at as "created_at!",
                fp.updated_at as "updated_at!",
                fp.created_by_id,
                fp.content,
                fp.sticky,
                ft.name as "forum_thread_name"
            FROM
                forum_threads AS ft
            JOIN
                forum_posts AS fp ON ft.id = fp.forum_thread_id
            WHERE
                ft.forum_sub_category_id = $1
            ORDER BY
                ft.id DESC, fp.created_at ASC
            LIMIT $2
            "#,
            forum_sub_category_id,
            limit as i32
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThreadsFirstPost)
    }

    pub async fn search_forum_threads(
        &self,
        form: &ForumSearchQuery,
    ) -> Result<PaginatedResults<ForumSearchResult>> {
        let limit = form.page as i64 * form.page_size as i64;
        let offset = (form.page - 1) as i64 * form.page_size as i64;

        let results = sqlx::query_as!(
            ForumSearchResult,
            r#"
            SELECT
                t.name AS thread_name,
                t.id AS thread_id,
                p.content AS post,
                p.id AS post_id,
                p.created_at AS post_created_at,
                p.created_by_id AS post_created_by_id,
                u.username AS post_created_by_username,
                s.name AS sub_category_name,
                s.id AS sub_category_id,
                c.name AS category_name,
                c.id AS category_id
            FROM forum_threads t
            JOIN LATERAL (
                SELECT p.*
                FROM forum_posts p
                WHERE p.forum_thread_id = t.id
                ORDER BY p.created_at DESC
                LIMIT 1
            ) p ON TRUE
            JOIN users u ON u.id = p.created_by_id
            JOIN forum_sub_categories s ON s.id = t.forum_sub_category_id
            JOIN forum_categories c ON c.id = s.forum_category_id

            WHERE $1::TEXT IS NULL OR t.name ILIKE '%' || $1 || '%'

            ORDER BY p.created_at DESC

            LIMIT $2 OFFSET $3;
            "#,
            form.thread_name,
            limit,
            offset
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThreadsFirstPost)?;

        let total_results = sqlx::query!(
            "SELECT COUNT(*) AS total FROM forum_threads WHERE name ILIKE '%' || $1 || '%'",
            form.thread_name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForumThreads)?
        .total
        .unwrap_or(0);

        Ok(PaginatedResults {
            results,
            total_items: total_results,
            page: form.page,
            page_size: form.page_size,
        })
    }
}
