use actix_web::{web, HttpResponse};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::Arcadia;
use arcadia_common::error::Result;
use arcadia_storage::models::torrent::TorrentSearchResults;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetTopTorrentsQuery {
    period: String,
    amount: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get top torrent",
    tag = "Torrent",
    path = "/api/torrents/top",
    params(GetTopTorrentsQuery),
    responses(
        (status = 200, description = "Top torrents found (and their title/edition group), sorted by amount of users who seeded at some point in time", body=TorrentSearchResults),
    )
)]
pub async fn exec(
    query: web::Query<GetTopTorrentsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let search_results = arc
        .pool
        .find_top_torrents(&query.period, query.amount)
        .await?;

    Ok(HttpResponse::Ok().json(search_results))
}
