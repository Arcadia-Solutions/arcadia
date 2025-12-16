use crate::{
    connection_pool::ConnectionPool,
    models::user::{
        EditedUser, EditedUserClass, PublicUser, UserClass, UserCreatedUserClass,
        UserCreatedUserWarning, UserMinimal, UserPermission, UserSettings, UserWarning,
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::PgPool;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn find_user_profile(&self, id: &i32) -> Result<PublicUser> {
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
                    class_name,
                    class_locked,
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

    pub async fn update_last_seen(&self, id: i32) -> Result<()> {
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

    pub async fn update_user(&self, user_id: i32, edited_user: &EditedUser) -> Result<()> {
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

    pub async fn get_user_settings(&self, user_id: i32) -> Result<UserSettings> {
        let user_settings = sqlx::query_as!(
            UserSettings,
            r#"
                SELECT css_sheet_name
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::UserWithIdNotFound(user_id))?;

        Ok(user_settings)
    }

    pub async fn update_user_settings(&self, user_id: i32, settings: &UserSettings) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET css_sheet_name = $2
                WHERE id = $1
            "#,
            user_id,
            settings.css_sheet_name
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn create_user_warning(
        &self,
        current_user_id: i32,
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

    pub async fn find_user_warnings(&self, user_id: i32) -> Vec<UserWarning> {
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

    pub async fn is_user_banned(&self, user_id: i32) -> Result<bool> {
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
            SELECT id, passkey FROM users
            "#
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(users)
    }

    pub async fn create_user_class(&self, user_class: &UserCreatedUserClass) -> Result<UserClass> {
        sqlx::query_as!(
            UserClass,
            r#"
                INSERT INTO user_classes (
                    name,
                    default_permissions,
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                RETURNING
                    name,
                    default_permissions as "default_permissions: Vec<UserPermission>",
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
            "#,
            user_class.name,
            &user_class.default_permissions as &[UserPermission],
            user_class.automatic_promotion,
            user_class.automatic_demotion,
            user_class.promotion_allowed_while_warned,
            user_class.required_account_age_in_days,
            user_class.required_ratio,
            user_class.required_torrent_uploads,
            user_class.required_torrent_uploads_in_unique_title_groups,
            user_class.required_uploaded,
            user_class.required_torrent_snatched,
            user_class.required_downloaded,
            user_class.required_forum_posts,
            user_class.required_forum_posts_in_unique_threads,
            user_class.required_title_group_comments,
            user_class.required_seeding_size
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code() == Some(std::borrow::Cow::Borrowed("23505"))
            {
                return Error::UserClassAlreadyExists;
            }
            Error::CouldNotCreateUserClass(e)
        })
    }

    pub async fn get_user_class_by_name(&self, name: &str) -> Result<UserClass> {
        sqlx::query_as!(
            UserClass,
            r#"
                SELECT
                    name,
                    default_permissions as "default_permissions: Vec<UserPermission>",
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                FROM user_classes
                WHERE name = $1
            "#,
            name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::UserClassNotFound(name.to_string()))
    }

    pub async fn get_all_user_classes(&self) -> Result<Vec<UserClass>> {
        sqlx::query_as!(
            UserClass,
            r#"
                SELECT
                    name,
                    default_permissions as "default_permissions: Vec<UserPermission>",
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                FROM user_classes
                ORDER BY name
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::from)
    }

    pub async fn update_user_class(
        &self,
        old_name: &str,
        edited_class: &EditedUserClass,
    ) -> Result<UserClass> {
        sqlx::query_as!(
            UserClass,
            r#"
                UPDATE user_classes
                SET
                    name = $2,
                    default_permissions = $3,
                    automatic_promotion = $4,
                    automatic_demotion = $5,
                    promotion_allowed_while_warned = $6,
                    required_account_age_in_days = $7,
                    required_ratio = $8,
                    required_torrent_uploads = $9,
                    required_torrent_uploads_in_unique_title_groups = $10,
                    required_uploaded = $11,
                    required_torrent_snatched = $12,
                    required_downloaded = $13,
                    required_forum_posts = $14,
                    required_forum_posts_in_unique_threads = $15,
                    required_title_group_comments = $16,
                    required_seeding_size = $17
                WHERE name = $1
                RETURNING
                    name,
                    default_permissions as "default_permissions: Vec<UserPermission>",
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
            "#,
            old_name,
            edited_class.name,
            &edited_class.default_permissions as &[UserPermission],
            edited_class.automatic_promotion,
            edited_class.automatic_demotion,
            edited_class.promotion_allowed_while_warned,
            edited_class.required_account_age_in_days,
            edited_class.required_ratio,
            edited_class.required_torrent_uploads,
            edited_class.required_torrent_uploads_in_unique_title_groups,
            edited_class.required_uploaded,
            edited_class.required_torrent_snatched,
            edited_class.required_downloaded,
            edited_class.required_forum_posts,
            edited_class.required_forum_posts_in_unique_threads,
            edited_class.required_title_group_comments,
            edited_class.required_seeding_size
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                Error::UserClassNotFound(old_name.to_string())
            } else if let sqlx::Error::Database(ref db_err) = e {
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                    return Error::UserClassAlreadyExists;
                }
                Error::CouldNotUpdateUserClass(e)
            } else {
                Error::CouldNotUpdateUserClass(e)
            }
        })
    }

    pub async fn delete_user_class(&self, name: &str, target_class_name: &str) -> Result<()> {
        // Verify target class exists
        self.get_user_class_by_name(target_class_name).await?;

        // Migrate all users from the deleted class to the target class
        sqlx::query!(
            r#"
                UPDATE users
                SET class_name = $2
                WHERE class_name = $1
            "#,
            name,
            target_class_name
        )
        .execute(self.borrow())
        .await?;

        // Delete the user class
        let result = sqlx::query!(r#"DELETE FROM user_classes WHERE name = $1"#, name)
            .execute(self.borrow())
            .await
            .map_err(Error::CouldNotDeleteUserClass)?;

        if result.rows_affected() == 0 {
            return Err(Error::UserClassNotFound(name.to_string()));
        }

        Ok(())
    }

    pub async fn count_users_in_class(&self, class_name: &str) -> Result<i64> {
        let result = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM users WHERE class_name = $1"#,
            class_name
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(result.unwrap_or(0))
    }

    pub async fn update_user_permissions(
        &self,
        user_id: i32,
        permissions: &[UserPermission],
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET permissions = $2
                WHERE id = $1
            "#,
            user_id,
            permissions as &[UserPermission]
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn lock_user_class(&self, user_id: i32, locked: bool) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET class_locked = $2
                WHERE id = $1
            "#,
            user_id,
            locked
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn change_user_class(&self, user_id: i32, class_name: &str) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET class_name = $2
                WHERE id = $1
            "#,
            user_id,
            class_name
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }
}
