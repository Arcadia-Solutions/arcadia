use crate::{
    connection_pool::ConnectionPool,
    models::{
        invitation::Invitation,
        user::{APIKey, Login, Register, User, UserCreatedAPIKey},
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
        open_signups: &bool,
    ) -> Result<User> {
        let mut rng = rand::rng();

        let passkey = rng.random::<u128>();

        let passkey_upper = (passkey >> 64) as i64;
        let passkey_lower = passkey as i64;

        // Check username availability first
        if self.does_username_exist(&user.username).await? {
            return Err(Error::UsernameAlreadyExists);
        }

        let settings =
            serde_json::json!({"site_appearance":{"item_detail_layout": "sidebar_right"}});

        let registered_user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (username, email, password_hash, registered_from_ip, settings, passkey_upper, passkey_lower)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#,
            &user.username,
            &user.email,
            password_hash,
            from_ip,
            settings,
            passkey_upper,
            passkey_lower
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateUser)?;

        if !*open_signups {
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
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE username = $1
            "#,
            login.username
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::WrongUsernameOrPassword)?;

        let parsed_hash = PasswordHash::new(&user.password_hash);

        Argon2::default()
            .verify_password(login.password.as_bytes(), &parsed_hash.unwrap())
            .map_err(|_| Error::WrongUsernameOrPassword)?;

        Ok(user)
    }

    pub async fn find_user_id_with_api_key(&self, api_key: &str) -> Result<User> {
        let user = sqlx::query_as!(
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

    pub async fn find_user_with_id(&self, id: i64) -> Result<User> {
        sqlx::query_as!(
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
        current_user_id: i64,
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
}
