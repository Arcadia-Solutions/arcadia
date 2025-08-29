use crate::{
    services::auth::{AUTH_TOKEN_LONG_DURATION, REFRESH_TOKEN_DURATION},
    Arcadia,
};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{Claims, LoginResponse, RefreshToken},
    redis::RedisPoolInterface,
};
use chrono::prelude::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[utoipa::path(
    post,
    operation_id = "Refresh token",
    tag = "Auth",
    path = "/api/auth/refresh-token",
    responses(
        (status = 200, description = "Successfully refreshed the token", body=LoginResponse),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: web::Data<Arcadia<R>>,
    form: web::Json<RefreshToken>,
) -> Result<HttpResponse> {
    let old_refresh_token = decode::<Claims>(
        &form.refresh_token,
        &DecodingKey::from_secret(arc.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| Error::InvalidOrExpiredRefreshToken)?;

    let is_invalidated = arc
        .auth
        .is_invalidated(old_refresh_token.claims.sub, old_refresh_token.claims.iat)
        .await?;
    if is_invalidated {
        return Err(Error::InvalidatedToken);
    }

    let user = arc
        .pool
        .find_user_with_id(old_refresh_token.claims.sub)
        .await?;
    if user.banned {
        return Err(Error::AccountBanned);
    }

    let now = Utc::now();
    let token_claims = Claims {
        sub: old_refresh_token.claims.sub,
        iat: now.timestamp(),
        exp: (Utc::now() + *AUTH_TOKEN_LONG_DURATION).timestamp(),
        class: old_refresh_token.claims.class.clone(),
    };

    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(arc.jwt_secret.as_bytes()),
    )
    .unwrap();

    let refresh_token_claims = Claims {
        sub: old_refresh_token.claims.sub,
        exp: (now + *REFRESH_TOKEN_DURATION).timestamp(),
        iat: now.timestamp(),
        class: old_refresh_token.claims.class.clone(),
    };

    let refresh_token = encode(
        &Header::default(),
        &refresh_token_claims,
        &EncodingKey::from_secret(arc.jwt_secret.as_bytes()),
    )
    .map_err(Error::JwtError)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "refresh_token": refresh_token
    })))
}
