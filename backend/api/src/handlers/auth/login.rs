use crate::{
    services::auth::{AUTH_TOKEN_LONG_DURATION, AUTH_TOKEN_SHORT_DURATION, REFRESH_TOKEN_DURATION},
    Arcadia,
};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{Claims, Login, LoginResponse},
    redis::RedisPoolInterface,
};
use chrono::prelude::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

#[utoipa::path(
    post,
    operation_id = "Login",
    tag = "Auth",
    path = "/api/auth/login",
    responses(
        (status = 200, description = "Successfully logged in", body=LoginResponse),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: web::Data<Arcadia<R>>,
    user_login: web::Json<Login>,
) -> Result<HttpResponse> {
    let user = arc.pool.find_user_with_password(&user_login).await?;

    if user.banned {
        return Err(Error::AccountBanned);
    }

    let mut token_expiration_date = Utc::now();
    let mut refresh_token = String::from("");
    let now = Utc::now();

    if !user_login.remember_me {
        token_expiration_date += *AUTH_TOKEN_SHORT_DURATION;
    } else {
        token_expiration_date += *AUTH_TOKEN_LONG_DURATION;

        let refresh_token_expiration_date = Utc::now() + *REFRESH_TOKEN_DURATION;
        let refresh_token_claims = Claims {
            sub: user.id,
            exp: refresh_token_expiration_date.timestamp(),
            iat: now.timestamp(),
            class: user.class.clone(),
        };
        refresh_token = encode(
            &Header::default(),
            &refresh_token_claims,
            &EncodingKey::from_secret(arc.jwt_secret.as_bytes()),
        )
        .unwrap();
    }

    let token_claims = Claims {
        sub: user.id,
        exp: token_expiration_date.timestamp(),
        iat: now.timestamp(),
        class: user.class,
    };

    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(arc.jwt_secret.as_bytes()),
    )
    .map_err(Error::JwtError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "refresh_token": refresh_token
    })))
}
