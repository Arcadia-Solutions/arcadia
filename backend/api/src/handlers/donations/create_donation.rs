use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{donation::Donation, donation::UserCreatedDonation, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create donation",
    tag = "Donation",
    path = "/api/donations",
    security(("http" = ["Bearer"])),
    request_body = UserCreatedDonation,
    responses(
        (status = 201, description = "Successfully created donation", body = Donation),
        (status = 403, description = "Forbidden - Insufficient permissions")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    donation: Json<UserCreatedDonation>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditArcadiaSettings)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::EditArcadiaSettings
        )));
    }

    let donation = arc.pool.create_donation(&donation, user.sub).await?;

    Ok(HttpResponse::Created().json(donation))
}
