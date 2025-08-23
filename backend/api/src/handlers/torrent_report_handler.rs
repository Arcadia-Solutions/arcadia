use actix_web::{web, HttpResponse};
use arcadia_storage::models::torrent_report::{TorrentReport, UserCreatedTorrentReport};

use crate::{handlers::User, Arcadia};
use arcadia_common::error::Result;

#[utoipa::path(
    post,
    path = "/api/report/torrent",
    responses(
        (status = 200, description = "Torrent successfully reported", body=TorrentReport),
    )
)]
pub async fn add_torrent_report(
    form: web::Json<UserCreatedTorrentReport>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let report = arc.pool.report_torrent(&form, &current_user).await?;

    Ok(HttpResponse::Ok().json(report))
}
