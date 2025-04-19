use crate::{
    Error, Result,
    models::{
        invitation::Invitation,
        user::{Login, Register, User},
    },
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use rand::Rng;
use sqlx::{PgPool, types::ipnetwork::IpNetwork};

pub async fn create_user(
    pool: &PgPool,
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

    let settings = serde_json::json!({"site_appearance":{"item_detail_layout": "sidebar_right"}});

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
    .fetch_one(pool)
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
        .execute(pool)
        .await;
    }

    Ok(registered_user)
}

pub async fn find_user_with_password(pool: &PgPool, login: &Login) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users
            WHERE username = $1
        "#,
        login.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::WrongUsernameOrPassword)?;

    let parsed_hash = PasswordHash::new(&user.password_hash);

    Argon2::default()
        .verify_password(login.password.as_bytes(), &parsed_hash.unwrap())
        .map_err(|_| Error::WrongUsernameOrPassword)?;

    Ok(user)
}

pub async fn find_user_with_id(pool: &PgPool, id: i64) -> Result<User> {
    sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::WrongUsernameOrPassword)
}
