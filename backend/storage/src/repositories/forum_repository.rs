use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        forum::{
            EditedForumCategory, EditedForumPost, EditedForumSubCategory, EditedForumThread,
            ForumCategory, ForumCategoryHierarchy, ForumCategoryLite, ForumPost,
            ForumPostAndThreadName, ForumPostHierarchy, ForumSearchQuery, ForumSearchResult,
            ForumSubCategory, ForumSubCategoryHierarchy, ForumThread, ForumThreadEnriched,
            ForumThreadPostLite, GetForumThreadPostsQuery, UserCreatedForumCategory,
            UserCreatedForumPost, UserCreatedForumSubCategory, UserCreatedForumThread,
        },
        user::{UserLite, UserLiteAvatar},
    },
};
use arcadia_common::error::{Error, Result};
use chrono::{DateTime, Local, Utc};
use serde_json::Value;
use sqlx::{prelude::FromRow, PgPool};
use std::borrow::Borrow;

#[derive(FromRow)]
struct DBImportSubCategoryWithLatestPost {
    id: i32,
    name: String,
    threads_amount: i64,
    posts_amount: i64,
    forbidden_classes: Vec<String>,
    forum_category_id: i32,
    category_name: String,
    latest_post_id: Option<i64>,
    thread_id: Option<i64>,
    thread_name: Option<String>,
    latest_post_created_at: Option<DateTime<Utc>>,
    user_id: Option<i32>,
    username: Option<String>,
    warned: Option<bool>,
    banned: Option<bool>,
}

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

impl ConnectionPool {
    pub async fn create_forum_post(
        &self,
        forum_post: &UserCreatedForumPost,
        current_user_id: i32,
    ) -> Result<ForumPost> {
        if forum_post.content.trim().is_empty() {
            return Err(Error::ForumPostEmpty);
        }

        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let thread = sqlx::query!(
            r#"SELECT locked FROM forum_threads WHERE id = $1"#,
            forum_post.forum_thread_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumPost)?;

        if thread.locked {
            return Err(Error::ForumThreadLocked);
        }

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

        sqlx::query!(
            r#"
            UPDATE users
            SET forum_posts = forum_posts + 1
            WHERE id = $1;
            "#,
            current_user_id
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
        if forum_thread.name.trim().is_empty() {
            return Err(Error::ForumThreadNameEmpty);
        }

        if forum_thread.first_post.content.trim().is_empty() {
            return Err(Error::ForumPostEmpty);
        }

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

        sqlx::query!(
            r#"
            UPDATE users
            SET forum_threads = forum_threads + 1
            WHERE id = $1;
            "#,
            current_user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateForumThread)?;

        tx.commit().await?;

        // Create the first post (this will increment posts_amount)
        self.create_forum_post(&forum_thread.first_post, current_user_id)
            .await?;

        // Subscribe the creator to the thread
        self.create_subscription_forum_thread_posts(created_forum_thread.id, current_user_id)
            .await?;

        // Fetch and return the updated thread with correct posts_amount and subscription status
        let updated_thread = sqlx::query_as!(
            ForumThread,
            r#"SELECT * FROM forum_threads WHERE id = $1"#,
            created_forum_thread.id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThread)?;

        Ok(updated_thread)
    }

    pub async fn update_forum_thread(
        &self,
        edited_thread: &EditedForumThread,
        user_id: i32,
    ) -> Result<ForumThreadEnriched> {
        if edited_thread.name.trim().is_empty() {
            return Err(Error::BadRequest("Thread name cannot be empty".to_string()));
        }

        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        // Get current sub-category id
        let old_sub_category_id = sqlx::query_scalar!(
            r#"SELECT forum_sub_category_id FROM forum_threads WHERE id = $1"#,
            edited_thread.id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotFindForumThread)?;

        // If sub-category is changing, update counters on both sub-categories
        if old_sub_category_id != edited_thread.forum_sub_category_id {
            // Decrement counters on the old sub-category
            sqlx::query!(
                r#"
                UPDATE forum_sub_categories
                SET threads_amount = threads_amount - 1,
                    posts_amount = posts_amount - (SELECT posts_amount FROM forum_threads WHERE id = $2)
                WHERE id = $1
                "#,
                old_sub_category_id,
                edited_thread.id
            )
            .execute(&mut *tx)
            .await
            .map_err(Error::CouldNotUpdateForumThread)?;

            // Increment counters on the new sub-category
            sqlx::query!(
                r#"
                UPDATE forum_sub_categories
                SET threads_amount = threads_amount + 1,
                    posts_amount = posts_amount + (SELECT posts_amount FROM forum_threads WHERE id = $2)
                WHERE id = $1
                "#,
                edited_thread.forum_sub_category_id,
                edited_thread.id
            )
            .execute(&mut *tx)
            .await
            .map_err(Error::CouldNotUpdateForumThread)?;
        }

        // Update the thread
        let updated_thread = sqlx::query_as!(
            ForumThreadEnriched,
            r#"
            WITH updated_row AS (
                UPDATE forum_threads
                SET name = $1, sticky = $2, locked = $3, forum_sub_category_id = $4
                WHERE id = $5
                RETURNING *
            )
            SELECT
                ur.id,
                ur.forum_sub_category_id,
                ur.name,
                ur.created_at,
                ur.created_by_id,
                ur.posts_amount,
                ur.sticky,
                ur.locked,
                fsc.name AS forum_sub_category_name,
                fc.name AS forum_category_name,
                fc.id AS forum_category_id,
                (sft.id IS NOT NULL) AS "is_subscribed!"
            FROM updated_row ur
            JOIN
                forum_sub_categories AS fsc ON ur.forum_sub_category_id = fsc.id
            JOIN
                forum_categories AS fc ON fsc.forum_category_id = fc.id
            LEFT JOIN
                subscriptions_forum_thread_posts AS sft
                ON sft.forum_thread_id = ur.id AND sft.user_id = $6
            "#,
            edited_thread.name,
            edited_thread.sticky,
            edited_thread.locked,
            edited_thread.forum_sub_category_id,
            edited_thread.id,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotUpdateForumThread)?;

        tx.commit().await?;

        Ok(updated_thread)
    }

    pub async fn find_forum_cateogries_hierarchy(&self) -> Result<Vec<ForumCategoryHierarchy>> {
        // Query all categories at once
        let categories = sqlx::query_as!(
            ForumCategoryLite,
            "SELECT id, name FROM forum_categories ORDER BY id"
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumSubCategory)?;

        // Query all subcategories with their latest posts in one query
        let sub_categories_data = sqlx::query_as!(
            DBImportSubCategoryWithLatestPost,
            r#"
            SELECT fsc.id, fsc.name, fsc.threads_amount, fsc.posts_amount, fsc.forbidden_classes,
                   fsc.forum_category_id, fc.name AS category_name,
                   fp.id AS "latest_post_id?", ft.id AS "thread_id?", ft.name AS "thread_name?", fp.created_at AS "latest_post_created_at?",
                   u.id AS "user_id?", u.username AS "username?", u.warned AS "warned?", u.banned AS "banned?"
            FROM forum_sub_categories fsc
            INNER JOIN forum_categories fc ON fsc.forum_category_id = fc.id
            LEFT JOIN LATERAL (
                SELECT fp.id, fp.created_at, fp.created_by_id, fp.forum_thread_id
                FROM forum_posts fp
                JOIN forum_threads ft_inner ON fp.forum_thread_id = ft_inner.id
                WHERE ft_inner.forum_sub_category_id = fsc.id
                ORDER BY fp.created_at DESC LIMIT 1
            ) AS fp ON TRUE
            LEFT JOIN forum_threads ft ON fp.forum_thread_id = ft.id
            LEFT JOIN users u ON fp.created_by_id = u.id
            ORDER BY fsc.forum_category_id, fsc.name
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumSubCategory)?;

        // Build hierarchy by grouping subcategories by category
        use std::collections::HashMap;
        let mut category_map: HashMap<i32, Vec<ForumSubCategoryHierarchy>> = HashMap::new();

        for sc in sub_categories_data {
            let sub_category = ForumSubCategoryHierarchy {
                id: sc.id,
                name: sc.name,
                threads_amount: sc.threads_amount,
                posts_amount: sc.posts_amount,
                forbidden_classes: sc.forbidden_classes,
                latest_post_in_thread: match (
                    sc.latest_post_id,
                    sc.thread_id,
                    sc.thread_name,
                    sc.latest_post_created_at,
                    sc.user_id,
                    sc.username,
                    sc.warned,
                    sc.banned,
                ) {
                    (
                        Some(id),
                        Some(thread_id),
                        Some(name),
                        Some(created_at),
                        Some(user_id),
                        Some(username),
                        Some(warned),
                        Some(banned),
                    ) => Some(ForumThreadPostLite {
                        id,
                        thread_id,
                        name,
                        created_at: created_at.with_timezone(&Local),
                        created_by: UserLite {
                            id: user_id,
                            username,
                            warned,
                            banned,
                        },
                    }),
                    _ => None,
                },
                threads: None,
                category: ForumCategoryLite {
                    id: sc.forum_category_id,
                    name: sc.category_name,
                },
            };
            category_map
                .entry(sc.forum_category_id)
                .or_default()
                .push(sub_category);
        }

        // Build final result with categories in order
        let forum_categories = categories
            .into_iter()
            .map(|category| ForumCategoryHierarchy {
                id: category.id,
                name: category.name,
                sub_categories: category_map.remove(&category.id).unwrap_or_default(),
            })
            .collect();

        Ok(forum_categories)
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
                                            'sticky', ft.sticky,
                                            'locked', ft.locked,
                                            'created_by', json_build_object(
                                                'id', u_thread.id,
                                                'username', u_thread.username,
                                                'warned', u_thread.warned,
                                                'banned', u_thread.banned
                                            ),
                                            'latest_post', json_build_object(
                                                'id', fp_latest.id,
                                                'thread_id', ft.id,
                                                'name', ft.name,
                                                'created_at', fp_latest.created_at,
                                                'created_by', json_build_object(
                                                    'id', u_post.id,
                                                    'username', u_post.username,
                                                    'warned', u_post.warned,
                                                    'banned', u_post.banned
                                                )
                                            )
                                        ) ORDER BY ft.created_at DESC
                                    ),
                                    '[]'::json
                                )
                            FROM
                                forum_threads ft
                            JOIN
                                users u_thread ON ft.created_by_id = u_thread.id
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
        .fetch_optional(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumSubCategory)?;

        match forum_sub_category {
            Some(record) => Ok(record.result_json.unwrap_or(serde_json::json!({}))),
            None => Err(Error::CouldNotFindForumSubCategory(
                sqlx::Error::RowNotFound,
            )),
        }
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

    pub async fn find_forum_category(&self, category_id: i32) -> Result<ForumCategory> {
        sqlx::query_as!(
            ForumCategory,
            r#"SELECT * FROM forum_categories WHERE id = $1"#,
            category_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::ForumCategoryNotFound)
    }

    pub async fn find_forum_sub_category_raw(
        &self,
        sub_category_id: i32,
    ) -> Result<ForumSubCategory> {
        sqlx::query_as!(
            ForumSubCategory,
            r#"
            SELECT
                fsc.id,
                fsc.forum_category_id,
                fsc.name,
                fsc.created_at,
                fsc.created_by_id,
                fsc.forbidden_classes,
                (SELECT COUNT(*) FROM forum_threads ft WHERE ft.forum_sub_category_id = fsc.id) AS "threads_amount!",
                (SELECT COUNT(*) FROM forum_posts fp JOIN forum_threads ft ON fp.forum_thread_id = ft.id WHERE ft.forum_sub_category_id = fsc.id) AS "posts_amount!"
            FROM forum_sub_categories fsc
            WHERE fsc.id = $1
            "#,
            sub_category_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumSubCategory)
    }

    pub async fn create_forum_category(
        &self,
        forum_category: &UserCreatedForumCategory,
        current_user_id: i32,
    ) -> Result<ForumCategory> {
        if forum_category.name.trim().is_empty() {
            return Err(Error::ForumCategoryNameEmpty);
        }

        let created_category = sqlx::query_as!(
            ForumCategory,
            r#"
                INSERT INTO forum_categories (name, created_by_id)
                VALUES ($1, $2)
                RETURNING *
            "#,
            forum_category.name,
            current_user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateForumCategory)?;

        Ok(created_category)
    }

    pub async fn update_forum_category(
        &self,
        edited_category: &EditedForumCategory,
    ) -> Result<ForumCategory> {
        if edited_category.name.trim().is_empty() {
            return Err(Error::ForumCategoryNameEmpty);
        }

        let updated_category = sqlx::query_as!(
            ForumCategory,
            r#"
                UPDATE forum_categories
                SET name = $1
                WHERE id = $2
                RETURNING *
            "#,
            edited_category.name,
            edited_category.id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::ForumCategoryNotFound,
            _ => Error::CouldNotUpdateForumCategory(e),
        })?;

        Ok(updated_category)
    }

    pub async fn create_forum_sub_category(
        &self,
        forum_sub_category: &UserCreatedForumSubCategory,
        current_user_id: i32,
    ) -> Result<ForumSubCategory> {
        if forum_sub_category.name.trim().is_empty() {
            return Err(Error::ForumSubCategoryNameEmpty);
        }

        let created_sub_category = sqlx::query_as!(
            ForumSubCategory,
            r#"
                INSERT INTO forum_sub_categories (name, forum_category_id, created_by_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            forum_sub_category.name,
            forum_sub_category.forum_category_id,
            current_user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateForumSubCategory)?;

        Ok(created_sub_category)
    }

    pub async fn update_forum_sub_category(
        &self,
        edited_sub_category: &EditedForumSubCategory,
    ) -> Result<ForumSubCategory> {
        if edited_sub_category.name.trim().is_empty() {
            return Err(Error::ForumSubCategoryNameEmpty);
        }

        let updated_sub_category = sqlx::query_as!(
            ForumSubCategory,
            r#"
                UPDATE forum_sub_categories
                SET name = $1
                WHERE id = $2
                RETURNING *
            "#,
            edited_sub_category.name,
            edited_sub_category.id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::ForumSubCategoryNotFound,
            _ => Error::CouldNotUpdateForumSubCategory(e),
        })?;

        Ok(updated_sub_category)
    }

    pub async fn delete_forum_category(&self, category_id: i32) -> Result<()> {
        // Check if category has any sub-categories
        let sub_category_count = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM forum_sub_categories WHERE forum_category_id = $1"#,
            category_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotDeleteForumCategory)?
        .unwrap_or(0);

        if sub_category_count > 0 {
            return Err(Error::ForumCategoryHasSubCategories);
        }

        // Delete the category
        let result = sqlx::query!(r#"DELETE FROM forum_categories WHERE id = $1"#, category_id)
            .execute(self.borrow())
            .await
            .map_err(Error::CouldNotDeleteForumCategory)?;

        if result.rows_affected() == 0 {
            return Err(Error::ForumCategoryNotFound);
        }

        Ok(())
    }

    pub async fn delete_forum_sub_category(&self, sub_category_id: i32) -> Result<()> {
        // Check if sub-category has any threads
        let thread_count = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM forum_threads WHERE forum_sub_category_id = $1"#,
            sub_category_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotDeleteForumSubCategory)?
        .unwrap_or(0);

        if thread_count > 0 {
            return Err(Error::ForumSubCategoryHasThreads);
        }

        // Delete the sub-category
        let result = sqlx::query!(
            r#"DELETE FROM forum_sub_categories WHERE id = $1"#,
            sub_category_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotDeleteForumSubCategory)?;

        if result.rows_affected() == 0 {
            return Err(Error::ForumSubCategoryNotFound);
        }

        Ok(())
    }

    pub async fn delete_forum_thread(&self, thread_id: i64) -> Result<()> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        // Get thread info (sub_category_id, posts_amount, created_by_id) before deletion
        let thread_info = sqlx::query!(
            r#"
            SELECT forum_sub_category_id, posts_amount, created_by_id
            FROM forum_threads
            WHERE id = $1
            "#,
            thread_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::CouldNotFindForumThread(e),
            _ => Error::CouldNotDeleteForumThread(e),
        })?;

        // Delete all posts in the thread (cascade delete)
        sqlx::query!(
            r#"DELETE FROM forum_posts WHERE forum_thread_id = $1"#,
            thread_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotDeleteForumThread)?;

        // Delete the thread
        let result = sqlx::query!(r#"DELETE FROM forum_threads WHERE id = $1"#, thread_id)
            .execute(&mut *tx)
            .await
            .map_err(Error::CouldNotDeleteForumThread)?;

        if result.rows_affected() == 0 {
            return Err(Error::CouldNotFindForumThread(sqlx::Error::RowNotFound));
        }

        // Update sub-category counters
        sqlx::query!(
            r#"
            UPDATE forum_sub_categories
            SET threads_amount = threads_amount - 1,
                posts_amount = posts_amount - $2
            WHERE id = $1
            "#,
            thread_info.forum_sub_category_id,
            thread_info.posts_amount
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotDeleteForumThread)?;

        // Decrement user's forum_threads counter
        sqlx::query!(
            r#"
            UPDATE users
            SET forum_threads = forum_threads - 1
            WHERE id = $1;
            "#,
            thread_info.created_by_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotDeleteForumThread)?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn delete_forum_post(&self, post_id: i64) -> Result<()> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        // Get post info (thread_id and created_by_id) before deletion
        let post_info = sqlx::query!(
            r#"SELECT forum_thread_id, created_by_id FROM forum_posts WHERE id = $1"#,
            post_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::CouldNotFindForumPost(e),
            _ => Error::CouldNotDeleteForumPost(e),
        })?;

        // Delete the post
        let result = sqlx::query!(r#"DELETE FROM forum_posts WHERE id = $1"#, post_id)
            .execute(&mut *tx)
            .await
            .map_err(Error::CouldNotDeleteForumPost)?;

        if result.rows_affected() == 0 {
            return Err(Error::CouldNotFindForumPost(sqlx::Error::RowNotFound));
        }

        // Decrement thread post counter
        sqlx::query!(
            r#"
            UPDATE forum_threads
            SET posts_amount = posts_amount - 1
            WHERE id = $1
            "#,
            post_info.forum_thread_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotDeleteForumPost)?;

        // Decrement sub-category post counter
        sqlx::query!(
            r#"
            UPDATE forum_sub_categories
            SET posts_amount = posts_amount - 1
            WHERE id = (SELECT forum_sub_category_id FROM forum_threads WHERE id = $1)
            "#,
            post_info.forum_thread_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotDeleteForumPost)?;

        // Decrement user's forum_posts counter
        sqlx::query!(
            r#"
            UPDATE users
            SET forum_posts = forum_posts - 1
            WHERE id = $1;
            "#,
            post_info.created_by_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::CouldNotDeleteForumPost)?;

        tx.commit().await?;

        Ok(())
    }
}
