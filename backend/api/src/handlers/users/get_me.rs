use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::{
    torrent::{
        TorrentSearch, TorrentSearchOrder, TorrentSearchSortField, TorrentSearchTitleGroup,
        TorrentSearchTorrent,
    },
    user::Profile,
};
use serde_json::json;

#[utoipa::path(
    get,
    operation_id = "Get me",
    tag = "User",
    path = "/api/users/me",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the user's profile", body=Profile),
    )
)]
pub async fn exec(arc: web::Data<Arcadia>, user: Authdata) -> Result<HttpResponse> {
    let mut current_user = arc.pool.find_user_with_id(user.sub).await?;
    current_user.password_hash = String::from("");
    let peers = arc.pool.get_user_peers(current_user.id).await;
    let user_warnings = arc.pool.find_user_warnings(current_user.id).await;
    let search_title_group = TorrentSearchTitleGroup {
        name: String::from(""),
        include_empty_groups: false,
    };
    let search_torrent = TorrentSearchTorrent {
        reported: None,
        staff_checked: None,
        created_by_id: Some(current_user.id),
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
        .search_torrents(&torrent_search, Some(current_user.id))
        .await?;
    torrent_search.torrent.snatched_by_id = Some(current_user.id);
    torrent_search.torrent.created_by_id = None;
    torrent_search.sort_by = TorrentSearchSortField::TorrentSnatchedAt;
    let snatched_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(current_user.id))
        .await?;
    let unread_conversations_amount = arc
        .pool
        .find_unread_conversations_amount(current_user.id)
        .await?;
    let unread_notifications_amount = arc
        .pool
        .find_unread_notifications_amount(current_user.id)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
            "user": current_user,
            "peers":peers,
            "user_warnings": user_warnings,
            "unread_conversations_amount": unread_conversations_amount,
            "unread_notifications_amount":unread_notifications_amount,
            "last_five_uploaded_torrents": uploaded_torrents.get("title_groups").unwrap(),
            "last_five_snatched_torrents": snatched_torrents.get("title_groups").unwrap()
    })))
}
