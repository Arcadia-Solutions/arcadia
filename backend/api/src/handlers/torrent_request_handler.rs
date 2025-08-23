use crate::handlers::User;
use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_request::{
    TorrentRequest, TorrentRequestAndAssociatedData, TorrentRequestFill,
    TorrentRequestWithTitleGroupLite, UserCreatedTorrentRequest,
};
use serde_json::json;

use serde::Deserialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

#[utoipa::path(
    post,
    path = "/api/torrent-request",
    responses(
        (status = 200, description = "Successfully created the torrent_request", body=TorrentRequest),
    )
)]
pub async fn add_torrent_request(
    mut torrent_request: web::Json<UserCreatedTorrentRequest>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let torrent_request = arc
        .pool
        .create_torrent_request(&mut torrent_request, &current_user)
        .await?;

    Ok(HttpResponse::Created().json(torrent_request))
}

#[utoipa::path(
    post,
    path = "/api/torrent-request/fill",
    responses(
        (status = 200, description = "Successfully filled the torrent request"),
    )
)]
pub async fn fill_torrent_request(
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

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct SearchTorrentRequestsQuery {
    pub title_group_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/api/search/torrent-request",
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
pub async fn search_torrent_requests(
    arc: web::Data<Arcadia>,
    query: web::Query<SearchTorrentRequestsQuery>,
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

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTorrentRequestQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/torrent-request",
    params(GetTorrentRequestQuery),
    responses(
        (status = 200, description = "Successfully got the torrent request with associated data", body=TorrentRequestAndAssociatedData),
    )
)]
pub async fn get_torrent_request(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTorrentRequestQuery>,
    _current_user: User,
) -> Result<HttpResponse> {
    let torrent_request = arc.pool.find_torrent_request_hierarchy(query.id).await?;

    Ok(HttpResponse::Ok().json(torrent_request))
}
