use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_shared::tracker::models::env::ArcadiaSettingsForTracker;
use log::info;

use crate::Tracker;

pub async fn exec(arc: Data<Tracker>, settings: Json<ArcadiaSettingsForTracker>) -> HttpResponse {
    info!(
        "Updating settings: upload_factor={}, download_factor={}",
        settings.global_upload_factor, settings.global_download_factor
    );

    let mut current = arc.settings.write();
    current.global_upload_factor = settings.global_upload_factor;
    current.global_download_factor = settings.global_download_factor;

    HttpResponse::Ok().finish()
}
