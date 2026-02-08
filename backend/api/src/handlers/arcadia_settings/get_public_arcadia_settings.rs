use crate::Arcadia;
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::arcadia_settings::PublicArcadiaSettings, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get public Arcadia settings",
    tag = "Arcadia Settings",
    path = "/api/arcadia-settings/public",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully retrieved Arcadia public settings", body=PublicArcadiaSettings),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let settings = arc.settings.lock().unwrap().clone();
    Ok(HttpResponse::Ok().json(PublicArcadiaSettings {
        global_download_factor: settings.global_download_factor,
        global_upload_factor: settings.global_upload_factor,
        open_signups: settings.open_signups,
        logo_subtitle: settings.logo_subtitle,
        bonus_points_alias: settings.bonus_points_alias,
        bonus_points_decimal_places: settings.bonus_points_decimal_places,
    }))
}
