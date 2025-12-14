use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserPermission, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get user permissions",
    tag = "User",
    path = "/api/users/{id}/permissions",
    security(("http" = ["Bearer"])),
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Successfully retrieved user permissions", body=Vec<UserPermission>),
        (status = 403, description = "Insufficient privileges"),
        (status = 404, description = "User not found"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    user_id: Path<i32>,
    current_user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(current_user.sub, &UserPermission::EditUserPermissions)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }

    let target_user = arc.pool.find_user_with_id(*user_id).await?;

    Ok(HttpResponse::Ok().json(target_user.permissions))
}
