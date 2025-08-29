use crate::handlers::User;
use crate::Arcadia;
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::torrent_request::TorrentRequestFill, redis::RedisPoolInterface};
use serde_json::json;

#[utoipa::path(
    post,
    operation_id = "Fill torrent request",
    tag = "Torrent Request",
    path = "/api/torrent-requests/fill",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully filled the torrent request"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    torrent_request_fill: Json<TorrentRequestFill>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    arc.pool
        .fill_torrent_request(
            torrent_request_fill.torrent_id,
            torrent_request_fill.torrent_request_id,
            current_user.id,
        )
        .await?;

    Ok(HttpResponse::Ok().json(json!({"result": "succes"})))
}
