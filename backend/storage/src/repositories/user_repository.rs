use crate::{
    connection_pool::ConnectionPool,
    models::user::{EditedUser, PublicUser, UserCreatedUserWarning, UserMinimal, UserWarning},
};
use arcadia_common::error::{Error, Result};
use sqlx::PgPool;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn find_user_profile(&self, id: &i64) -> Result<PublicUser> {
        sqlx::query_as!(
            PublicUser,
            r#"
                SELECT
                    id,
                    username,
                    avatar,
                    created_at,
                    description,
                    uploaded,
                    downloaded,
                    real_uploaded,
                    real_downloaded,
                    ratio,
                    required_ratio,
                    last_seen,
                    class,
                    forum_posts,
                    forum_threads,
                    torrent_comments,
                    request_comments,
                    artist_comments,
                    seeding,
                    leeching,
                    snatched,
                    seeding_size,
                    requests_filled,
                    collages_started,
                    requests_voted,
                    average_seeding_time,
                    invited,
                    invitations,
                    bonus_points,
                    warned,
                    banned
                FROM users
                WHERE id = $1
            "#,
            *id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::UserWithIdNotFound(*id))
    }

    pub async fn update_last_seen(&self, id: i64) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET last_seen = NOW()
                WHERE id = $1
            "#,
            id
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn update_user(&self, user_id: i64, edited_user: &EditedUser) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET avatar = $2, description = $3, email = $4
                WHERE id = $1
            "#,
            user_id,
            edited_user.avatar,
            edited_user.description,
            edited_user.email
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn create_user_warning(
        &self,
        current_user_id: i64,
        user_warning: &UserCreatedUserWarning,
    ) -> Result<UserWarning> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET warned = true,
                banned = CASE
                    WHEN $2 IS TRUE THEN TRUE
                    ELSE banned
                END
                WHERE id = $1
            "#,
            user_warning.user_id,
            user_warning.ban
        )
        .execute(&mut *tx)
        .await?;

        let user_warning = sqlx::query_as!(
            UserWarning,
            r#"
                INSERT INTO user_warnings (user_id, expires_at, reason, created_by_id, ban)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            user_warning.user_id,
            user_warning.expires_at,
            user_warning.reason,
            current_user_id,
            user_warning.ban
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateGift)?;

        tx.commit().await?;

        Ok(user_warning)
    }

    pub async fn find_user_warnings(&self, user_id: i64) -> Vec<UserWarning> {
        sqlx::query_as!(
            UserWarning,
            r#"
                SELECT * FROM user_warnings
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.borrow())
        .await
        .expect("failed to get user warnings")
    }

    pub async fn is_user_banned(&self, user_id: i64) -> Result<bool> {
        let result = sqlx::query_scalar!("SELECT banned FROM users WHERE id = $1", user_id)
            .fetch_optional(self.borrow())
            .await?;

        let Some(banned) = result else {
            return Ok(true);
        };

        Ok(banned)
    }

    pub async fn find_registered_users(&self) -> Result<Vec<UserMinimal>> {
        let users = sqlx::query_as!(
            UserMinimal,
            r#"
            SELECT id, passkey_upper, passkey_lower FROM users
            "#
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(users)
    }
}
