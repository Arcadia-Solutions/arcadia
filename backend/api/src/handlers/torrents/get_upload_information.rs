use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use arcadia_storage::redis::RedisPoolInterface;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{handlers::User, Arcadia};
use arcadia_common::{error::Result, services::torrent_service::get_announce_url};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadInformation {
    announce_url: String,
}

#[utoipa::path(
    get,
    operation_id = "Get upload information",
    tag = "Torrent",
    path = "/api/torrents/upload-info",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Information related to uploading", body=UploadInformation),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    let announce_url = get_announce_url(
        current_user.passkey_upper,
        current_user.passkey_lower,
        arc.tracker.url.as_ref(),
    );

    Ok(HttpResponse::Ok().json(UploadInformation { announce_url }))
}
