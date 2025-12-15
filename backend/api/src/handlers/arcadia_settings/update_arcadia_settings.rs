use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_shared::tracker::models::env::ArcadiaSettingsForTracker;
use arcadia_storage::{
    models::{arcadia_settings::ArcadiaSettings, user::UserPermission},
    redis::RedisPoolInterface,
};
use log::warn;
use reqwest::Client;

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

    // Notify tracker of settings change
    let client = Client::new();
    let mut url = arc.env.tracker.url_internal.clone();
    url.path_segments_mut()
        .unwrap()
        .push("api")
        .push("settings");

    let payload = ArcadiaSettingsForTracker {
        global_upload_factor: updated_settings.global_upload_factor,
        global_download_factor: updated_settings.global_download_factor,
    };

    if let Err(e) = client
        .put(url)
        .header("x-api-key", arc.env.tracker.api_key.clone())
        .json(&payload)
        .send()
        .await
    {
        warn!("Failed to update settings in tracker: {}", e);
    }

    Ok(HttpResponse::Ok().json(updated_settings))
}
