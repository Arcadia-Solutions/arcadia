use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        donation::{Donation, EditedDonation},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit donation",
    tag = "Donation",
    path = "/api/donations",
    security(
        ("http" = ["Bearer"])
    ),
    request_body = EditedDonation,
    responses(
        (status = 200, description = "Successfully edited the donation", body=Donation),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    request: Json<EditedDonation>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let existing_donation = arc.pool.find_donation_by_id(request.id).await?;

    let can_create = arc
        .pool
        .user_has_permission(user.sub, &UserPermission::CreateDonation)
        .await?;
    let is_creator = existing_donation.created_by_id == user.sub;
    let has_edit_permission = arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditDonation)
        .await?;

    if !((can_create && is_creator) || has_edit_permission) {
        return Err(Error::InsufficientPermissions(
            "Cannot edit this donation".to_string(),
        ));
    }

    arc.pool.find_user_with_id(request.donated_by_id).await?;

    if request.amount <= 0.0 {
        return Err(Error::BadRequest(
            "Donation amount must be positive".to_string(),
        ));
    }

    let updated_donation = arc.pool.update_donation(&request).await?;

    Ok(HttpResponse::Ok().json(updated_donation))
}
