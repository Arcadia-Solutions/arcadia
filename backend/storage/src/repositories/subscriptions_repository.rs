use crate::connection_pool::ConnectionPool;
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_subscription_forum_thread_posts(
        &self,
        thread_id: i64,
        current_user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO subscriptions_forum_thread_posts (user_id, forum_thread_id)
                VALUES ($1, $2)
            "#,
            current_user_id,
            thread_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotCreateSubscription)?;

        Ok(())
    }

    pub async fn delete_subscription_forum_thread_posts(
        &self,
        thread_id: i64,
        current_user_id: i32,
    ) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                DELETE FROM subscriptions_forum_thread_posts
                WHERE forum_thread_id = $1 AND user_id = $2;
            "#,
            thread_id,
            current_user_id
        )
        .execute(self.borrow())
        .await?;

        // TODO: check result.rows_affected()
        Ok(())
    }

    pub async fn create_subscription_title_group_torrents(
        &self,
        title_group_id: i32,
        current_user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                   INSERT INTO subscriptions_title_group_torrents (user_id, title_group_id)
                   VALUES ($1, $2)
               "#,
            current_user_id,
            title_group_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotCreateSubscription)?;

        Ok(())
    }

    pub async fn delete_subscription_title_group_torrents(
        &self,
        title_group_id: i32,
        current_user_id: i32,
    ) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                DELETE FROM subscriptions_title_group_torrents
                WHERE title_group_id = $1 AND user_id = $2;
            "#,
            title_group_id,
            current_user_id
        )
        .execute(self.borrow())
        .await?;

        // TODO: check result.rows_affected()
        Ok(())
    }
}
