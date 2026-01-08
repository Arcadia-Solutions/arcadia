use actix_web::{web::Data, HttpResponse};
use arcadia_storage::redis::RedisPoolInterface;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::{error::Result, services::torrent_service::get_announce_url};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadInformation {
    announce_url: String,
    upload_page_top_text: Option<String>,
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
    user: Authdata,
) -> Result<HttpResponse> {
    let current_user = arc.pool.find_user_with_id(user.sub).await?;
    let announce_url = get_announce_url(current_user.passkey, arc.tracker.url.as_ref());
    let settings = arc.settings.lock().unwrap();

    Ok(HttpResponse::Ok().json(UploadInformation {
        announce_url,
        upload_page_top_text: settings.upload_page_top_text.clone(),
    }))
}
