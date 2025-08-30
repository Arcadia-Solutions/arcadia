use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_report::{TorrentReport, UserCreatedTorrentReport};

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
pub async fn exec(
    form: web::Json<UserCreatedTorrentReport>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let report = arc.pool.report_torrent(&form, user.sub).await?;

    Ok(HttpResponse::Ok().json(report))
}
