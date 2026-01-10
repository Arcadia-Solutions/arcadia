use crate::{
    connection_pool::ConnectionPool,
    models::{
        arcadia_settings::ArcadiaSettings,
        common::PaginatedResults,
        invitation::Invitation,
        unauthorized_access::{SearchUnauthorizedAccessQuery, UnauthorizedAccess},
        user::{APIKey, Login, Register, User, UserCreatedAPIKey, UserLiteAvatar, UserPermission},
    },
};
use arcadia_common::error::{Error, Result};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use rand::{
    distr::{Alphanumeric, SampleString},
    rng, Rng,
};
use sqlx::{types::ipnetwork::IpNetwork, PgPool};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn does_username_exist(&self, username: &str) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    pub async fn create_user(
        &self,
        user: &Register,
        from_ip: IpNetwork,
        password_hash: &str,
        invitation: &Invitation,
        arcadia_settings: &ArcadiaSettings,
    ) -> Result<User> {
        let rng = rand::rng();

        // TODO: check if the passkey already exists
        let passkey: String = rng
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        // Check username availability first
        if self.does_username_exist(&user.username).await? {
            return Err(Error::UsernameAlreadyExists);
        }

        let registered_user = sqlx::query_as_unchecked!(
            User,
            r#"
                INSERT INTO users (username, email, password_hash, registered_from_ip, passkey, class_name, css_sheet_name)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#,
            &user.username,
            &user.email,
            password_hash,
            from_ip,
            passkey,
            arcadia_settings.user_class_name_on_signup,
            arcadia_settings.default_css_sheet_name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateUser)?;

        // Assign class permissions to the new user
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET permissions = (SELECT new_permissions FROM user_classes WHERE name = $2)
                WHERE id = $1
            "#,
            registered_user.id,
            arcadia_settings.user_class_name_on_signup
        )
        .execute(self.borrow())
        .await?;

        if !arcadia_settings.open_signups {
            // TODO: check this properly
            let _ = sqlx::query!(
                r#"
                UPDATE invitations SET receiver_id = $1 WHERE id = $2;
                "#,
                registered_user.id,
                invitation.id
            )
            .execute(self.borrow())
            .await;
        }

        Ok(registered_user)
    }

    pub async fn find_user_with_password(&self, login: &Login) -> Result<User> {
        let user = sqlx::query_as_unchecked!(
            User,
            r#"
                SELECT * FROM users
                WHERE username = $1
            "#,
            login.username
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| {
            log::debug!("Error fetching user: {:?}", e);
            Error::WrongUsernameOrPassword
        })?;

        let parsed_hash = PasswordHash::new(&user.password_hash);

        Argon2::default()
            .verify_password(login.password.as_bytes(), &parsed_hash.unwrap())
            .map_err(|_| Error::WrongUsernameOrPassword)?;

        Ok(user)
    }

    pub async fn find_user_id_with_api_key(&self, api_key: &str) -> Result<User> {
        let user = sqlx::query_as_unchecked!(
            User,
            r#"
            SELECT u.*
            FROM users u
            JOIN api_keys ak ON u.id = ak.user_id
            WHERE ak.value = $1 AND u.banned = FALSE;
            "#,
            api_key
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::InvalidAPIKeyOrBanned)?;

        Ok(user)
    }

    pub async fn find_user_with_id(&self, id: i32) -> Result<User> {
        sqlx::query_as_unchecked!(
            User,
            r#"
                SELECT * FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::WrongUsernameOrPassword)
    }

    pub async fn create_api_key(
        &self,
        created_api_key: &UserCreatedAPIKey,
        current_user_id: i32,
    ) -> Result<APIKey> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        loop {
            let api_key: String = Alphanumeric.sample_string(&mut rng(), 40);

            let api_key = sqlx::query_as!(
                APIKey,
                r#"
                INSERT INTO api_keys (name, value, user_id)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
                created_api_key.name,
                api_key,
                current_user_id
            )
            .fetch_one(&mut *tx)
            .await;

            match api_key {
                Ok(api_key) => {
                    tx.commit().await?;

                    return Ok(api_key);
                }
                Err(api_key_error) => {
                    return Err(match &api_key_error {
                        sqlx::Error::Database(database_error) => {
                            let code = database_error.code();
                            // 23505 is the code for "unique violation", which means we didn't generate a unique API key
                            if let Some(code) = code
                                && code == "23505"
                            {
                                // Try again (jump to next iteration of loop)
                                continue;
                            }

                            Error::CouldNotCreateAPIKey(api_key_error)
                        }
                        _ => Error::CouldNotCreateAPIKey(api_key_error),
                    });
                }
            }
        }
    }

    pub async fn user_has_permission(
        &self,
        user_id: i32,
        permission: &UserPermission,
    ) -> Result<bool> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE id = $1 AND $2 = ANY(permissions)
            ) as "exists!"
            "#,
            user_id,
            permission as &UserPermission
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(result)
    }

    pub async fn require_permission(
        &self,
        user_id: i32,
        permission: &UserPermission,
        path: &str,
    ) -> Result<()> {
        let has_permission = self.user_has_permission(user_id, permission).await?;

        if !has_permission {
            // Log unauthorized access
            let _ = sqlx::query!(
                r#"
                INSERT INTO unauthorized_accesses (user_id, missing_permission, path)
                VALUES ($1, $2, $3)
                "#,
                user_id,
                permission as &UserPermission,
                path
            )
            .execute(self.borrow())
            .await;

            return Err(Error::InsufficientPermissions(format!("{:?}", permission)));
        }

        Ok(())
    }

    pub async fn find_unauthorized_accesses(
        &self,
        query: SearchUnauthorizedAccessQuery,
    ) -> Result<PaginatedResults<UnauthorizedAccess>> {
        let total_items: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM unauthorized_accesses
            WHERE ($1::INT IS NULL OR user_id = $1)
              AND created_at >= $2
              AND created_at <= $3
              AND ($4::user_permissions_enum IS NULL OR missing_permission = $4)
            "#,
            query.user_id,
            query.from_date,
            query.to_date,
            query.permission.clone() as Option<UserPermission>
        )
        .fetch_one(self.borrow())
        .await?
        .unwrap_or(0);

        let rows = sqlx::query!(
            r#"
            SELECT
                ua.id,
                ua.created_at,
                u.id as user_id,
                u.username,
                u.banned,
                u.avatar,
                u.warned,
                ua.missing_permission as "missing_permission: UserPermission",
                ua.path
            FROM unauthorized_accesses ua
            JOIN users u ON ua.user_id = u.id
            WHERE ($1::INT IS NULL OR ua.user_id = $1)
              AND ua.created_at >= $2
              AND ua.created_at <= $3
              AND ($4::user_permissions_enum IS NULL OR ua.missing_permission = $4)
            ORDER BY
              CASE WHEN $5 = 'missing_permission' AND $6 = 'asc' THEN ua.missing_permission END ASC,
              CASE WHEN $5 = 'missing_permission' AND $6 = 'desc' THEN ua.missing_permission END DESC,
              CASE WHEN $5 = 'created_at' AND $6 = 'asc' THEN ua.created_at END ASC,
              CASE WHEN $5 = 'created_at' AND $6 = 'desc' THEN ua.created_at END DESC
            OFFSET ($7 - 1) * LEAST($8, 100)
            LIMIT LEAST($8, 100)
            "#,
            query.user_id,
            query.from_date,
            query.to_date,
            query.permission as Option<UserPermission>,
            query.sort_by_column.to_string(),
            query.sort_by_direction.to_string(),
            query.page as i32,
            query.page_size as i32
        )
        .fetch_all(self.borrow())
        .await?;

        let results = rows
            .into_iter()
            .map(|row| UnauthorizedAccess {
                id: row.id,
                created_at: row.created_at,
                user: UserLiteAvatar {
                    id: row.user_id,
                    username: row.username,
                    banned: row.banned,
                    avatar: row.avatar,
                    warned: row.warned,
                },
                missing_permission: row.missing_permission,
                path: row.path,
            })
            .collect();

        Ok(PaginatedResults {
            results,
            total_items,
            page: query.page as u32,
            page_size: query.page_size.min(100) as u32,
        })
    }
}
