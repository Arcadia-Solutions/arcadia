use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{donation::Donation, donation::EditedDonation, user::UserClass},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit donation",
    tag = "Donation",
    path = "/api/donations/{id}",
    params(
        ("id" = i64, Path, description = "Donation ID")
    ),
    security(("http" = ["Bearer"])),
    request_body = EditedDonation,
    responses(
        (status = 200, description = "Successfully updated donation", body = Donation),
        (status = 403, description = "Forbidden - Only staff members can edit donations"),
        (status = 404, description = "Not Found - Donation not found")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    id: Path<i64>,
    donation: Json<EditedDonation>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    let donation = arc.pool.update_donation(*id, &donation).await?;

    Ok(HttpResponse::Ok().json(donation))
}
