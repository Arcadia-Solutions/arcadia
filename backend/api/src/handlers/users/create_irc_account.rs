use crate::{
    middlewares::auth_middleware::Authdata,
    services::irc_service::{generate_and_hash_irc_password, IrcService},
    Arcadia,
};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::redis::RedisPoolInterface;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct IrcAccountResponse {
    pub irc_password: String,
}

#[utoipa::path(
    post,
    operation_id = "Create IRC account",
    tag = "User",
    path = "/api/users/irc",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 201, description = "Successfully created IRC account", body = IrcAccountResponse),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    if !arc.env.ergo.is_enabled() {
        return Err(Error::IrcNotEnabled);
    }

    let current_user = arc.pool.find_user_with_id(user.sub).await?;

    if current_user.irc_password_hash.is_some() {
        return Err(Error::IrcAccountAlreadyExists);
    }

    let (irc_password, password_hash) = generate_and_hash_irc_password()?;

    let irc_service = IrcService::new(&arc)?;
    irc_service
        .create_account(&current_user.username, &irc_password)
        .await?;

    arc.pool
        .set_irc_password_hash(user.sub, &password_hash)
        .await?;

    Ok(HttpResponse::Created().json(IrcAccountResponse { irc_password }))
}
