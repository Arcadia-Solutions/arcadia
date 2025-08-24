use crate::handlers::User;
use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_request::TorrentRequestFill;
use serde_json::json;

#[utoipa::path(
    post,
    path = "/api/torrent-requests/fill",
    responses(
        (status = 200, description = "Successfully filled the torrent request"),
    )
)]
pub async fn exec(
    torrent_request_fill: web::Json<TorrentRequestFill>,
    arc: web::Data<Arcadia>,
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
