use crate::Arcadia;
use actix_web::web::{Data, Query};
use actix_web::HttpResponse;
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_request::TorrentRequestWithTitleGroupLite;

use arcadia_storage::redis::RedisPoolInterface;
use serde::Deserialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct SearchTorrentRequestsQuery {
    pub title_group_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[utoipa::path(
    get,
    operation_id = "Search torrent requests",
    tag = "Search",
    path = "/api/search/torrent-requests",
    params(
        ("title_group_name" = Option<String>, Query, description = "Name of the title group to search for"),
        ("tags" = Option<Vec<String>>, Query, description = "Tags to filter title groups by"),
        ("page" = Option<i64>, Query, description = "Page number (default 1)"),
        ("page_size" = Option<i64>, Query, description = "Results per page (default 50)"),
    ),
    responses(
        (status = 200, description = "List of torrent requests with associated title groups", body = [TorrentRequestWithTitleGroupLite]),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<SearchTorrentRequestsQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50);
    let results = arc
        .pool
        .search_torrent_requests(
            query.title_group_name.as_deref(),
            query.tags.as_deref(),
            page,
            page_size,
        )
        .await?;
    Ok(HttpResponse::Ok().json(results))
}
