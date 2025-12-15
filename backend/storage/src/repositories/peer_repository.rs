use crate::connection_pool::ConnectionPool;
use std::borrow::Borrow;

impl ConnectionPool {
    /// Removes inactive peers that haven't announced within the timeout period
    /// Returns the number of peers removed
    pub async fn remove_inactive_peers(
        &self,
        timeout_seconds: f64,
    ) -> std::result::Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM peers
            WHERE updated_at < NOW() - INTERVAL '1 second' * $1
            "#,
            timeout_seconds
        )
        .execute(self.borrow())
        .await?;

        Ok(result.rows_affected())
    }
}
