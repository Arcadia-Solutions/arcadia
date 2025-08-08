use crate::{
    Arcadia, Error, Result,
    models::{
        invitation::Invitation,
        user::{Claims, Login, LoginResponse, RefreshToken, Register, User},
    },
    repositories::{
        auth_repository::{create_user, find_user_id_with_api_key, find_user_with_password},
        invitation_repository::does_unexpired_invitation_exist,
    },
    services::email_service::EmailService,
};
use regex::Regex;
use lazy_static::lazy_static;
use actix_web::{HttpMessage as _, HttpRequest, HttpResponse, dev::ServiceRequest, web};

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]{4,20}$").unwrap();
}

pub fn validate_email(email: &str) -> Result<()> {
    if email.trim().is_empty() {
        return Err(Error::BadRequest("Email is required".to_string()));
    }
    if !EMAIL_REGEX.is_match(email) {
        return Err(Error::BadRequest("Please enter a valid email address".to_string()));
    }
    Ok(())
}

pub fn validate_username(username: &str) -> Result<()> {
    if username.trim().is_empty() {
        return Err(Error::BadRequest("Username is required".to_string()));
    }
    if !USERNAME_REGEX.is_match(username) {
        return Err(Error::BadRequest("Username must be 4-20 characters (letters, numbers, _-)".to_string()));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<()> {
    if password.is_empty() {
        return Err(Error::BadRequest("Password is required".to_string()));
    }
    if password.len() < 12 {
        return Err(Error::BadRequest("Password must be at least 12 characters long".to_string()));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(Error::BadRequest("Password must contain at least one uppercase letter".to_string()));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(Error::BadRequest("Password must contain at least one lowercase letter".to_string()));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(Error::BadRequest("Password must contain at least one number".to_string()));
    }
    Ok(())
}

pub fn validate_password_verification(password: &str, password_verify: &str) -> Result<()> {
    if password_verify.is_empty() {
        return Err(Error::BadRequest("Please confirm your password".to_string()));
    }
    if password != password_verify {
        return Err(Error::BadRequest("Passwords do not match".to_string()));
    }
    Ok(())
}
use actix_web_httpauth::extractors::bearer::BearerAuth;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::Duration;
use chrono::prelude::Utc;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use serde::Deserialize;
use sqlx::types::ipnetwork::IpNetwork;
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
    // Validate input fields
    validate_email(&new_user.email)?;
    validate_username(&new_user.username)?;
    validate_password(&new_user.password)?;
    validate_password_verification(&new_user.password, &new_user.password_verify)?;

    let invitation: Invitation;
    if !arc.is_open_signups() {
        let invitation_key = query
            .invitation_key
            .as_ref()
            .ok_or(Error::InvitationKeyRequired)?;

        invitation = does_unexpired_invitation_exist(&arc.pool, invitation_key).await?;

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

    let user = create_user(
        &arc.pool,
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
    let user = find_user_with_password(&arc.pool, &user_login).await?;

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

    let banned = crate::repositories::user_repository::is_user_banned(&arc.pool, user_id).await;

    if banned {
        return Err((actix_web::error::ErrorUnauthorized("account banned"), req));
    }

    let _ = crate::repositories::user_repository::update_last_seen(&arc.pool, user_id).await;

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

    let user_id = match find_user_id_with_api_key(&arc.pool, api_key).await {
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
