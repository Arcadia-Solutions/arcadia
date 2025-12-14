use crate::{handlers::tracker::binary_response, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::env::ArcadiaSettingsForTracker;
use arcadia_storage::redis::RedisPoolInterface;

pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let settings = arc.pool.get_arcadia_settings().await?;

    let settings = ArcadiaSettingsForTracker {
        global_download_factor: settings.global_download_factor,
        global_upload_factor: settings.global_upload_factor,
    };

    binary_response(&settings)
}
