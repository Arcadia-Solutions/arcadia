use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::torrent_request::{TorrentRequest, UserCreatedTorrentRequest},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create torrent request",
    tag = "Torrent Request",
    path = "/api/torrent-requests",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the torrent_request", body=TorrentRequest),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    mut torrent_request: Json<UserCreatedTorrentRequest>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let torrent_request = arc
        .pool
        .create_torrent_request(&mut torrent_request, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(torrent_request))
}
