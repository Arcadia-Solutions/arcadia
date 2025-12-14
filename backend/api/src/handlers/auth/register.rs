use crate::{
    services::{
        auth_service::{
            validate_email, validate_password, validate_password_verification, validate_username,
        },
        email_service::EmailService,
    },
    Arcadia,
};
use actix_web::{web, HttpRequest, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_shared::tracker::models::user::APIInsertUser;
use arcadia_storage::{
    models::{
        invitation::Invitation,
        user::{Register, User},
    },
    redis::RedisPoolInterface,
    sqlx::types::ipnetwork::IpNetwork,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use reqwest::Client;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterQuery {
    invitation_key: Option<String>,
}

#[utoipa::path(
    post,
    operation_id = "Register",
    tag = "Auth",
    path = "/api/auth/register",
    responses(
        (status = 200, description = "Successfully registered the user", body = User),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    new_user: web::Json<Register>,
    arc: web::Data<Arcadia<R>>,
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

    validate_email(&new_user.email)?;
    validate_username(&new_user.username)?;
    validate_password(&new_user.password)?;
    validate_password_verification(&new_user.password, &new_user.password_verify)?;

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
            &arc.env.user_class_name_on_signup,
        )
        .await?;

    // Inform tracker about the new user so it can accept announces immediately
    {
        let client = Client::new();
        let mut url = arc.env.tracker.url_internal.clone();
        url.path_segments_mut().unwrap().push("api").push("users");

        let payload = APIInsertUser {
            id: user.id as u32,
            passkey: user.passkey.parse().expect("invalid passkey format"),
        };

        // Fire and log; don't fail registration if tracker call fails
        if let Err(e) = client
            .put(url)
            .header("x-api-key", arc.env.tracker.api_key.clone())
            .json(&payload)
            .send()
            .await
        {
            log::warn!("Failed to upsert user in tracker: {}", e);
        }
    }

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
