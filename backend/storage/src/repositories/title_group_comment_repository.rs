use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        title_group_comment::{
            EditedTitleGroupComment, TitleGroupComment, TitleGroupCommentSearchQuery,
            TitleGroupCommentSearchResult, UserCreatedTitleGroupComment,
        },
        user::UserLite,
    },
};
use arcadia_common::error::{Error, Result};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool, Postgres, Transaction};
use std::borrow::Borrow;

#[derive(FromRow)]
struct DBTitleGroupCommentSearchResult {
    id: i64,
    content: String,
    created_at: DateTime<Utc>,
    title_group_id: i32,
    title_group_name: String,
    created_by_id: i32,
    created_by_username: String,
    created_by_warned: bool,
    created_by_banned: bool,
}

impl ConnectionPool {
    pub async fn create_title_group_comment(
        &self,
        title_group_comment: &UserCreatedTitleGroupComment,
        user_id: i32,
    ) -> Result<TitleGroupComment> {
        let mut tx: Transaction<'_, Postgres> = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let created_title_group_comment = sqlx::query_as!(
            TitleGroupComment,
            r#"
                WITH inserted_comment AS (
                    INSERT INTO title_group_comments (content, title_group_id, created_by_id,
                                                      refers_to_torrent_id, answers_to_comment_id)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING id, content, created_at, updated_at, created_by_id, title_group_id, locked, refers_to_torrent_id, answers_to_comment_id
                ),
                updated_user AS (
                    UPDATE users u
                    SET title_group_comments = u.title_group_comments + 1
                    WHERE u.id = (SELECT created_by_id FROM inserted_comment)
                )
                SELECT id, content, created_at, updated_at, created_by_id, title_group_id, locked, refers_to_torrent_id, answers_to_comment_id
                FROM inserted_comment
            "#,
            title_group_comment.content,
            title_group_comment.title_group_id,
            user_id,
            title_group_comment.refers_to_torrent_id,
            title_group_comment.answers_to_comment_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateTitleGroupComment)?;

        Self::notify_users_title_group_comments(
            &mut tx,
            title_group_comment.title_group_id,
            created_title_group_comment.id,
            user_id,
        )
        .await?;

        tx.commit().await?;

        Ok(created_title_group_comment)
    }

    pub async fn find_title_group_comment(&self, comment_id: i64) -> Result<TitleGroupComment> {
        let comment = sqlx::query_as!(
            TitleGroupComment,
            r#"
                SELECT id, content, created_at, updated_at, created_by_id, title_group_id, locked, refers_to_torrent_id, answers_to_comment_id FROM title_group_comments WHERE id = $1
            "#,
            comment_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindTitleGroupComment)?;

        Ok(comment)
    }

    pub async fn update_title_group_comment(
        &self,
        edited_comment: &EditedTitleGroupComment,
        comment_id: i64,
    ) -> Result<TitleGroupComment> {
        let updated_comment = sqlx::query_as!(
            TitleGroupComment,
            r#"
                UPDATE title_group_comments
                SET content = $2, locked = $3, updated_at = NOW()
                WHERE id = $1
                RETURNING id, content, created_at, updated_at, created_by_id, title_group_id, locked, refers_to_torrent_id, answers_to_comment_id
            "#,
            comment_id,
            edited_comment.content,
            edited_comment.locked
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| Error::ErrorWhileUpdatingTitleGroupComment(e.to_string()))?;

        Ok(updated_comment)
    }

    pub async fn search_title_group_comments(
        &self,
        form: &TitleGroupCommentSearchQuery,
    ) -> Result<PaginatedResults<TitleGroupCommentSearchResult>> {
        let limit = form.page_size as i64;
        let offset = (form.page - 1) as i64 * form.page_size as i64;

        let db_results = sqlx::query_as!(
            DBTitleGroupCommentSearchResult,
            r#"
            SELECT
                c.id,
                c.content,
                c.created_at,
                c.title_group_id,
                tg.name AS title_group_name,
                u.id AS created_by_id,
                u.username AS created_by_username,
                u.warned AS created_by_warned,
                u.banned AS created_by_banned
            FROM title_group_comments c
            JOIN users u ON u.id = c.created_by_id
            JOIN title_groups tg ON tg.id = c.title_group_id
            WHERE $1::TEXT IS NULL OR c.content ILIKE '%' || $1 || '%'
            ORDER BY c.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            form.content,
            limit,
            offset
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindTitleGroupComment)?;

        let results: Vec<TitleGroupCommentSearchResult> = db_results
            .into_iter()
            .map(|r| TitleGroupCommentSearchResult {
                id: r.id,
                content: r.content,
                created_at: r.created_at,
                title_group_id: r.title_group_id,
                title_group_name: r.title_group_name,
                created_by: UserLite {
                    id: r.created_by_id,
                    username: r.created_by_username,
                    warned: r.created_by_warned,
                    banned: r.created_by_banned,
                },
            })
            .collect();

        let total_results = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM title_group_comments
            WHERE $1::TEXT IS NULL OR content ILIKE '%' || $1 || '%'
            "#,
            form.content
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindTitleGroupComment)?
        .unwrap_or(0);

        Ok(PaginatedResults {
            results,
            total_items: total_results,
            page: form.page,
            page_size: form.page_size,
        })
    }

    pub async fn find_latest_title_group_comments(
        &self,
        limit: u32,
    ) -> Result<Vec<TitleGroupCommentSearchResult>> {
        let db_results = sqlx::query_as!(
            DBTitleGroupCommentSearchResult,
            r#"
            SELECT
                c.id,
                c.content,
                c.created_at,
                c.title_group_id,
                tg.name AS title_group_name,
                u.id AS created_by_id,
                u.username AS created_by_username,
                u.warned AS created_by_warned,
                u.banned AS created_by_banned
            FROM title_group_comments c
            JOIN users u ON u.id = c.created_by_id
            JOIN title_groups tg ON tg.id = c.title_group_id
            ORDER BY c.created_at DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotFindTitleGroupComment)?;

        let results: Vec<TitleGroupCommentSearchResult> = db_results
            .into_iter()
            .map(|r| TitleGroupCommentSearchResult {
                id: r.id,
                content: r.content,
                created_at: r.created_at,
                title_group_id: r.title_group_id,
                title_group_name: r.title_group_name,
                created_by: UserLite {
                    id: r.created_by_id,
                    username: r.created_by_username,
                    warned: r.created_by_warned,
                    banned: r.created_by_banned,
                },
            })
            .collect();

        Ok(results)
    }
}
