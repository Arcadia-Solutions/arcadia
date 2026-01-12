use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_shared::tracker::models::torrent::APIUpdateTorrentFactors;
use log::warn;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::Result;
use arcadia_storage::{models::user::UserPermission, redis::RedisPoolInterface};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditTorrentUpDownFactors {
    pub torrent_id: i32,
    pub upload_factor: i16,
    pub download_factor: i16,
}

#[utoipa::path(
    put,
    operation_id = "Edit torrent upload/download factors",
    tag = "Torrent",
    path = "/api/torrents/up-down-factors",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Torrent upload/download factors updated"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditTorrentUpDownFactors>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(
            user.sub,
            &UserPermission::EditTorrentUpDownFactors,
            req.path(),
        )
        .await?;

    arc.pool
        .update_torrent_up_down_factors(form.torrent_id, form.upload_factor, form.download_factor)
        .await?;

    // Notify tracker to update its in-memory state
    let client = Client::new();
    let mut url = arc.env.tracker.url_internal.clone();
    url.path_segments_mut()
        .unwrap()
        .push("api")
        .push("torrents")
        .push(&form.torrent_id.to_string())
        .push("up-down-factors");

    let payload = APIUpdateTorrentFactors {
        upload_factor: form.upload_factor,
        download_factor: form.download_factor,
    };

    if let Err(e) = client
        .put(url)
        .header("x-api-key", arc.env.tracker.api_key.clone())
        .json(&payload)
        .send()
        .await
    {
        warn!("Failed to update torrent factors in tracker: {}", e);
    }

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
