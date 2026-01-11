use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::OrderByDirection,
        torrent::{TorrentSearch, TorrentSearchOrderByColumn},
        user::PublicProfile,
    },
    redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUserQuery {
    id: i32,
}

#[utoipa::path(
    get,
    operation_id = "Get user",
    tag = "User",
    path = "/api/users",
    params(GetUserQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the user's profile", body=PublicProfile),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<GetUserQuery>,
    requesting_user: Authdata,
) -> Result<HttpResponse> {
    let user = arc.pool.find_user_profile(&query.id).await?;

    let mut torrent_search = TorrentSearch {
        title_group_name: None,
        title_group_include_empty_groups: false,
        title_group_content_type: Vec::new(),
        title_group_category: Vec::new(),
        edition_group_source: Vec::new(),
        torrent_video_resolution: Vec::new(),
        torrent_language: Vec::new(),
        torrent_reported: None,
        torrent_staff_checked: None,
        torrent_created_by_id: Some(query.id),
        torrent_snatched_by_id: None,
        page: 1,
        page_size: 5,
        order_by_column: TorrentSearchOrderByColumn::TorrentCreatedAt,
        order_by_direction: OrderByDirection::Desc,
        artist_id: None,
        collage_id: None,
        series_id: None,
    };
    let uploaded_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(requesting_user.sub))
        .await?;
    torrent_search.torrent_snatched_by_id = Some(query.id);
    torrent_search.torrent_created_by_id = None;
    torrent_search.order_by_column = TorrentSearchOrderByColumn::TorrentSnatchedAt;
    let snatched_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(user.id))
        .await?;

    Ok(HttpResponse::Ok().json(PublicProfile {
        user,
        last_five_uploaded_torrents: uploaded_torrents.results,
        last_five_snatched_torrents: snatched_torrents.results,
    }))
}
