use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{donation::DeletedDonation, user::UserPermission},
    redis::RedisPoolInterface,
};
use serde_json::json;

#[utoipa::path(
    delete,
    operation_id = "Delete donation",
    tag = "Donation",
    path = "/api/donations",
    security(
        ("http" = ["Bearer"])
    ),
    request_body = DeletedDonation,
    responses(
        (status = 200, description = "Donation deleted successfully"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    request: Json<DeletedDonation>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::DeleteDonation)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::DeleteDonation
        )));
    }

    arc.pool.find_donation_by_id(request.id).await?;

    arc.pool.delete_donation(request.id).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
