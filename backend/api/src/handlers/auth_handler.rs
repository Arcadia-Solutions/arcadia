use crate::{services::email_service::EmailService, Arcadia};
use actix_web::{dev::ServiceRequest, web, HttpMessage as _, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        invitation::Invitation,
        user::{Claims, Login, LoginResponse, RefreshToken, Register, User},
    },
    sqlx::types::ipnetwork::IpNetwork,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::prelude::Utc;
use chrono::Duration;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterQuery {
    invitation_key: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/register",
    responses(
        (status = 200, description = "Successfully registered the user", body = User),
    )
)]
pub async fn register(
    new_user: web::Json<Register>,
    arc: web::Data<Arcadia>,
    req: HttpRequest,
    query: web::Query<RegisterQuery>,
) -> Result<HttpResponse> {
    let invitation: Invitation;
    if !arc.is_open_signups() {
        let invitation_key = query
            .invitation_key
            .as_ref()
            .ok_or(Error::InvitationKeyRequired)?;

        invitation = arc
            .pool
            .does_unexpired_invitation_exist(invitation_key)
            .await?;

        // TODO: push check to db
        if invitation.receiver_id.is_some() {
            return Err(Error::InvitationKeyAlreadyUsed);
        }
    } else {
        invitation = Invitation::default();
    }

    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = arc
        .pool
        .create_user(
            &new_user,
            client_ip,
            &password_hash,
            &invitation,
            &arc.is_open_signups(),
        )
        .await?;

    // Send welcome email
    if let Ok(email_service) = EmailService::new(&arc) {
        if let Err(e) = email_service
            .send_registration_email(&new_user.email, &new_user.username)
            .await
        {
            // Log the error but don't fail the registration
            log::warn!("Failed to send welcome email to {}: {}", new_user.email, e);
        }
    } else {
        log::warn!("Email service not configured, skipping welcome email");
    }

    Ok(HttpResponse::Created().json(serde_json::json!(user)))
}

#[utoipa::path(
    post,
    path = "/api/login",
    responses(
        (status = 200, description = "Successfully logged in", body=LoginResponse),
    )
)]
pub async fn login(arc: web::Data<Arcadia>, user_login: web::Json<Login>) -> Result<HttpResponse> {
    let user = arc.pool.find_user_with_password(&user_login).await?;

    if user.banned {
        return Err(Error::AccountBanned);
    }

    let mut token_expiration_date = Utc::now();
    let mut refresh_token = String::from("");
    if !user_login.remember_me {
        token_expiration_date += Duration::hours(1);
    } else {
        token_expiration_date += Duration::days(1);

        let refresh_token_expiration_date = Utc::now() + Duration::days(90);
        let refresh_token_claims = Claims {
            sub: user.id,
            exp: refresh_token_expiration_date.timestamp() as usize,
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
        exp: token_expiration_date.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(arc.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "refresh_token": refresh_token
    })))
}

pub async fn authenticate_user(
    req: ServiceRequest,
    bearer: Option<BearerAuth>,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // These routes are explicitly not authenticated.
    if matches!(
        req.path(),
        "/api/login" | "/api/register" | "/api/refresh-token" | "/api/apply"
    ) {
        return Ok(req);
    }

    if let Some(bearer) = bearer {
        validate_bearer_auth(req, bearer).await
    } else if let Some(api_key) = req.headers().get("api_key") {
        let api_key = api_key.to_str().expect("api_key malformed").to_owned();
        validate_api_key(req, &api_key).await
    } else {
        Err((
            actix_web::error::ErrorUnauthorized(
                "authentication error, missing jwt token or API key",
            ),
            req,
        ))
    }
}

pub async fn validate_bearer_auth(
    req: ServiceRequest,
    bearer: BearerAuth,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let Some(arc) = req.app_data::<web::Data<Arcadia>>() else {
        return Err((
            actix_web::error::ErrorUnauthorized("authentication error"),
            req,
        ));
    };

    let decoding_key = DecodingKey::from_secret(arc.jwt_secret.as_ref());

    let validation = Validation::default();

    let token_data = match decode::<Claims>(bearer.token(), &decoding_key, &validation) {
        Ok(data) => data,
        Err(err) => {
            return Err((
                match err.kind() {
                    ErrorKind::ExpiredSignature => {
                        actix_web::error::ErrorUnauthorized("jwt token expired")
                    }
                    _ => actix_web::error::ErrorUnauthorized("authentication error"),
                },
                req,
            ));
        }
    };

    let user_id = token_data.claims.sub;

    let banned = arc.pool.is_user_banned(user_id).await;

    if banned {
        return Err((actix_web::error::ErrorUnauthorized("account banned"), req));
    }

    let _ = arc.pool.update_last_seen(user_id).await;

    req.extensions_mut()
        .insert(crate::handlers::UserId(user_id));

    Ok(req)
}

pub async fn validate_api_key(
    req: ServiceRequest,
    api_key: &str,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let Some(arc) = req.app_data::<web::Data<Arcadia>>() else {
        return Err((
            actix_web::error::ErrorUnauthorized("authentication error"),
            req,
        ));
    };

    let user_id = match arc.pool.find_user_id_with_api_key(api_key).await {
        Ok(id) => id,
        Err(e) => {
            return Err((actix_web::error::ErrorUnauthorized(e.to_string()), req));
        }
    };

    req.extensions_mut()
        .insert(crate::handlers::UserId(user_id));

    Ok(req)
}

#[utoipa::path(
    post,
    path = "/api/refresh-token",
    responses(
        (status = 200, description = "Successfully refreshed the token", body=LoginResponse),
    )
)]
pub async fn refresh_token(
    arc: web::Data<Arcadia>,
    form: web::Json<RefreshToken>,
) -> Result<HttpResponse> {
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
    .unwrap();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "refresh_token": refresh_token
    })))
}
