use actix_web::{web::Data, HttpResponse};

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::torrent::TorrentMinimal, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get registered torrents",
    tag = "Torrent",
    path = "/api/torrents/registered",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "All registered torrents", body=Vec<TorrentMinimal>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != "tracker" {
        return Err(Error::InsufficientPrivileges);
    };
    let torrents = arc.pool.find_registered_torrents().await?;

    Ok(HttpResponse::Ok().json(torrents))
}
