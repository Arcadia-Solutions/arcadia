use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::torrent_report::{TorrentReport, UserCreatedTorrentReport},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create torrent report",
    tag = "Torrent",
    path = "/api/torrents/reports",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Torrent successfully reported", body=TorrentReport),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<UserCreatedTorrentReport>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    let report = arc.pool.report_torrent(&form, &current_user).await?;

    Ok(HttpResponse::Ok().json(report))
}
