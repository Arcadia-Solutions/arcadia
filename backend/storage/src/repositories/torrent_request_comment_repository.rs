use crate::{
    connection_pool::ConnectionPool, models::torrent_request_comment::TorrentRequestComment,
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_torrent_request_comment(
        &self,
        torrent_request_id: i64,
        user_id: i32,
        content: &str,
    ) -> Result<TorrentRequestComment> {
        let created_torrent_request_comment = sqlx::query_as!(
            TorrentRequestComment,
            r#"
                WITH inserted_comment AS (
                    INSERT INTO torrent_request_comments (torrent_request_id, created_by_id, content)
                    VALUES ($1, $2, $3)
                    RETURNING *
                ),
                updated_user AS (
                    UPDATE users u
                    SET request_comments = u.request_comments + 1
                    WHERE u.id = (SELECT created_by_id FROM inserted_comment)
                )
                SELECT
                    inserted_comment.id,
                    inserted_comment.torrent_request_id,
                    inserted_comment.created_by_id,
                    inserted_comment.content,
                    inserted_comment.created_at,
                    inserted_comment.updated_at
                FROM inserted_comment
            "#,
            torrent_request_id,
            user_id,
            content
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateTorrentRequestComment)?;

        Ok(created_torrent_request_comment)
    }
}
