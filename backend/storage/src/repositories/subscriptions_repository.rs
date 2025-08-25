use crate::connection_pool::ConnectionPool;
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_subscription(
        &self,
        item_id: i64,
        item: &str, // TODO: should only be one of the existing columns of the table
        current_user_id: i64,
    ) -> Result<()> {
        sqlx::query(&format!(
            "
                INSERT INTO subscriptions ({item}_id, subscriber_id)
                VALUES ($1, $2)
            "
        ))
        .bind(item_id)
        .bind(current_user_id)
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotCreateSubscription)?;

        Ok(())
    }

    pub async fn delete_subscription(
        &self,
        item_id: i64,
        item: &str,
        current_user_id: i64,
    ) -> Result<()> {
        let _ = sqlx::query(&format!(
            "
                DELETE FROM subscriptions
                WHERE {item}_id = $1 AND subscriber_id = $2;
            "
        ))
        .bind(item_id)
        .bind(current_user_id)
        .execute(self.borrow())
        .await?;

        // TODO: check result.rows_affected()
        Ok(())
    }
}
