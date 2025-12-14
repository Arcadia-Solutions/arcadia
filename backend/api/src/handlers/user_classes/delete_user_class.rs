use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{DeleteUserClass, UserPermission},
    redis::RedisPoolInterface,
};
use serde_json::json;

#[utoipa::path(
    delete,
    operation_id = "Delete user class",
    tag = "User Class",
    path = "/api/user-classes/{name}",
    security(("http" = ["Bearer"])),
    params(
        ("name" = String, Path, description = "User class name to delete")
    ),
    request_body = DeleteUserClass,
    responses(
        (status = 200, description = "Successfully deleted user class and migrated users"),
        (status = 403, description = "Insufficient privileges"),
        (status = 404, description = "User class not found or target class not found"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    name: Path<String>,
    form: Json<DeleteUserClass>,
    user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::DeleteUserClass)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }

    // Delete user class and migrate users to target class
    arc.pool
        .delete_user_class(&name, &form.target_class_name)
        .await?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}
