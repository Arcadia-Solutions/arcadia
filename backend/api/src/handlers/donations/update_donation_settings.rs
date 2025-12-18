use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{donation::DonationSettings, donation::EditedDonationSettings, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Update donation settings",
    tag = "Donation",
    path = "/api/donations/settings",
    security(("http" = ["Bearer"])),
    request_body = EditedDonationSettings,
    responses(
        (status = 200, description = "Successfully updated donation settings", body = DonationSettings),
        (status = 403, description = "Forbidden - Insufficient permissions")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    settings: Json<EditedDonationSettings>,
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

    let updated_settings = arc.pool.update_donation_settings(&settings).await?;

    Ok(HttpResponse::Ok().json(updated_settings))
}
