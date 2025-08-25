use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::user::{Claims, LoginResponse, RefreshToken};
use chrono::prelude::Utc;
use chrono::Duration;
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
pub async fn exec(arc: web::Data<Arcadia>, form: web::Json<RefreshToken>) -> Result<HttpResponse> {
    let old_refresh_token = decode::<Claims>(
        &form.refresh_token,
        &DecodingKey::from_secret(arc.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| Error::InvalidOrExpiredRefreshToken)?;

    let token_claims = Claims {
        sub: old_refresh_token.claims.sub,
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(arc.jwt_secret.as_bytes()),
    )
    .unwrap();

    let refresh_token_claims = Claims {
        sub: old_refresh_token.claims.sub,
        exp: (Utc::now() + Duration::days(90)).timestamp() as usize,
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
