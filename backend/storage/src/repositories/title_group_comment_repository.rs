use crate::{
    connection_pool::ConnectionPool,
    models::title_group_comment::{
        EditedTitleGroupComment, TitleGroupComment, UserCreatedTitleGroupComment,
    },
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_title_group_comment(
        &self,
        title_group_comment: &UserCreatedTitleGroupComment,
        user_id: i32,
    ) -> Result<TitleGroupComment> {
        let created_title_group_comment = sqlx::query_as!(
            TitleGroupComment,
            r#"
                WITH inserted_comment AS (
                    INSERT INTO title_group_comments (content, title_group_id, created_by_id,
                                                      refers_to_torrent_id, answers_to_comment_id)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING *
                ),
                updated_user AS (
                    UPDATE users u
                    SET torrent_comments = u.torrent_comments + 1
                    WHERE u.id = (SELECT created_by_id FROM inserted_comment)
                )
                SELECT *
                FROM inserted_comment
            "#,
            title_group_comment.content,
            title_group_comment.title_group_id,
            user_id,
            title_group_comment.refers_to_torrent_id,
            title_group_comment.answers_to_comment_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateTitleGroupComment)?;

        Ok(created_title_group_comment)
    }

    pub async fn find_title_group_comment(&self, comment_id: i64) -> Result<TitleGroupComment> {
        let comment = sqlx::query_as!(
            TitleGroupComment,
            r#"
                SELECT * FROM title_group_comments WHERE id = $1
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
                RETURNING *
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
}
