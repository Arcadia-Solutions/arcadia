use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{arcadia_settings::ArcadiaSettings, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Update Arcadia settings",
    tag = "Arcadia Settings",
    path = "/api/arcadia-settings",
    security(
        ("http" = ["Bearer"])
    ),
    request_body = ArcadiaSettings,
    responses(
        (status = 200, description = "Successfully updated Arcadia settings", body=ArcadiaSettings),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    settings: Json<ArcadiaSettings>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditArcadiaSettings)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::EditArcadiaSettings
        )));
    }

    let updated_settings = arc.pool.update_arcadia_settings(&settings).await?;

    // Update the in-memory settings
    *arc.settings.lock().unwrap() = updated_settings.clone();

    Ok(HttpResponse::Ok().json(updated_settings))
}
