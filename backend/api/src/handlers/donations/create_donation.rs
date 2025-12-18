use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        donation::{Donation, UserCreatedDonation},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create donation",
    tag = "Donation",
    path = "/api/donations",
    security(
        ("http" = ["Bearer"])
    ),
    request_body = UserCreatedDonation,
    responses(
        (status = 201, description = "Successfully created the donation", body=Donation),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    request: Json<UserCreatedDonation>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::CreateDonation)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::CreateDonation
        )));
    }

    arc.pool.find_user_with_id(request.donated_by_id).await?;

    if request.amount <= 0.0 {
        return Err(Error::BadRequest(
            "Donation amount must be positive".to_string(),
        ));
    }

    let donation = arc.pool.create_donation(&request, user.sub).await?;

    Ok(HttpResponse::Created().json(donation))
}
