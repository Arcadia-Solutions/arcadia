use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{arcadia_settings::ArcadiaSettings, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Get Arcadia settings",
    tag = "Arcadia Settings",
    path = "/api/arcadia-settings",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully retrieved Arcadia settings", body=ArcadiaSettings),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditArcadiaSettings)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }

    let settings = arc.settings.lock().unwrap().clone();
    Ok(HttpResponse::Ok().json(settings))
}
