use crate::{
    middlewares::auth_middleware::Authdata, services::image_service::validate_image_url, Arcadia,
};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        user::UserPermission,
        user_badge::{EditedUserBadge, UserBadge},
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit user badge",
    tag = "User Badge",
    path = "/api/user-badges",
    security(("http" = ["Bearer"])),
    responses(
        (status = 200, description = "Successfully edited the user badge", body=UserBadge),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    edited: Json<EditedUserBadge>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::EditUserBadge, req.path())
        .await?;

    let approved_image_hosts = arc.settings.lock().unwrap().approved_image_hosts.clone();
    validate_image_url(&edited.image_url, &approved_image_hosts)?;

    let updated = arc.pool.update_user_badge(&edited).await?;
    Ok(HttpResponse::Ok().json(updated))
}
