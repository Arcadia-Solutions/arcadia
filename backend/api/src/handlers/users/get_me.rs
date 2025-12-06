use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::OrderByDirection,
        torrent::{TorrentSearch, TorrentSearchOrderByColumn},
        user::Profile,
    },
    redis::RedisPoolInterface,
};

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
pub async fn exec<R: RedisPoolInterface + 'static>(
    user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let mut current_user = arc.pool.find_user_with_id(user.sub).await?;
    current_user.password_hash = String::from("");
    // let peers = arc.pool.get_user_peers(current_user.id).await;
    let user_warnings = arc.pool.find_user_warnings(current_user.id).await;

    let mut torrent_search = TorrentSearch {
        title_group_name: None,
        title_group_include_empty_groups: false,
        torrent_reported: None,
        torrent_staff_checked: None,
        torrent_created_by_id: Some(current_user.id),
        torrent_snatched_by_id: None,
        page: 1,
        page_size: 5,
        order_by_column: TorrentSearchOrderByColumn::TorrentCreatedAt,
        order_by_direction: OrderByDirection::Desc,
        artist_id: None,
        collage_id: None,
    };
    let uploaded_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(current_user.id))
        .await?;
    torrent_search.torrent_snatched_by_id = Some(current_user.id);
    torrent_search.torrent_created_by_id = None;
    torrent_search.order_by_column = TorrentSearchOrderByColumn::TorrentSnatchedAt;
    let snatched_torrents = arc
        .pool
        .search_torrents(&torrent_search, Some(current_user.id))
        .await?;
    let unread_conversations_amount = arc
        .pool
        .find_unread_conversations_amount(current_user.id)
        .await?;
    let unread_notifications_amount_forum_thread_posts = arc
        .pool
        .find_unread_notifications_amount_forum_thread_posts(current_user.id)
        .await?;

    Ok(HttpResponse::Ok().json(Profile {
        user: current_user,
        peers: vec![],
        user_warnings,
        unread_conversations_amount,
        unread_notifications_amount_forum_thread_posts:
            unread_notifications_amount_forum_thread_posts as u32,
        last_five_uploaded_torrents: uploaded_torrents.results,
        last_five_snatched_torrents: snatched_torrents.results,
    }))
}
