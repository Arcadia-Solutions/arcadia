use actix_web::{web, HttpResponse};

use crate::{middlewares::jwt_middleware::JwtAuthData, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::torrent::TorrentMinimal;

#[utoipa::path(
    get,
    operation_id = "Get registered torrents",
    tag = "Torrent",
    path = "/api/torrents/registered",
    responses(
        (status = 200, description = "All registered torrents", body=Vec<TorrentMinimal>),
    )
)]
pub async fn exec(arc: web::Data<Arcadia>, user: JwtAuthData) -> Result<HttpResponse> {
    if user.class != "tracker" {
        return Err(Error::InsufficientPrivileges);
    };
    let torrents = arc.pool.find_registered_torrents().await?;

    Ok(HttpResponse::Ok().json(torrents))
}
