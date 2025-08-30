use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::{
    torrent::{
        TorrentSearch, TorrentSearchOrder, TorrentSearchSortField, TorrentSearchTitleGroup,
        TorrentSearchTorrent,
    },
    user::PublicProfile,
};
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUserQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get users",
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
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetUserQuery>,
    _: Authdata,
) -> Result<HttpResponse> {
    let user = arc.pool.find_user_profile(&query.id).await?;

    let search_title_group = TorrentSearchTitleGroup {
        name: String::from(""),
        include_empty_groups: false,
    };
    let search_torrent = TorrentSearchTorrent {
        reported: None,
        staff_checked: None,
        created_by_id: Some(query.id),
        snatched_by_id: None,
    };
    let mut torrent_search = TorrentSearch {
        title_group: search_title_group,
        torrent: search_torrent,
        page: 1,
        page_size: 5,
        sort_by: TorrentSearchSortField::TorrentCreatedAt,
        order: TorrentSearchOrder::Desc,
    };
    let uploaded_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(user.id))
        .await?;
    torrent_search.torrent.snatched_by_id = Some(query.id);
    torrent_search.torrent.created_by_id = None;
    torrent_search.sort_by = TorrentSearchSortField::TorrentSnatchedAt;
    let snatched_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(user.id))
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "user":user,
        "last_five_uploaded_torrents": uploaded_torrents.get("title_groups").unwrap(),
        "last_five_snatched_torrents": snatched_torrents.get("title_groups").unwrap()
    })))
}
