use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{donation::DonationSettings, user::UserClass},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Get donation settings",
    tag = "Donation",
    path = "/api/donations/settings",
    security(("http" = ["Bearer"])),
    responses(
        (status = 200, description = "Successfully retrieved donation settings", body = DonationSettings),
        (status = 403, description = "Forbidden - Only staff members can view donation settings")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    let settings = arc.pool.get_donation_settings().await?;

    Ok(HttpResponse::Ok().json(settings))
}
