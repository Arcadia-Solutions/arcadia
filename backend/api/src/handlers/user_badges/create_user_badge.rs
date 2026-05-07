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
        user_badge::{UserBadge, UserCreatedUserBadge},
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create user badge",
    tag = "User Badge",
    path = "/api/user-badges",
    security(("http" = ["Bearer"])),
    responses(
        (status = 201, description = "Successfully created the user badge", body=UserBadge),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    badge: Json<UserCreatedUserBadge>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::CreateUserBadge, req.path())
        .await?;

    let approved_image_hosts = arc.settings.lock().unwrap().approved_image_hosts.clone();
    validate_image_url(&badge.image_url, &approved_image_hosts)?;

    let created = arc.pool.create_user_badge(&badge, user.sub).await?;
    Ok(HttpResponse::Created().json(created))
}
