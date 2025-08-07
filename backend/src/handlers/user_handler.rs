use crate::{
    Arcadia, Error, Result,
    models::{
        torrent::{
            TorrentSearch, TorrentSearchOrder, TorrentSearchSortField, TorrentSearchTitleGroup,
            TorrentSearchTorrent,
        },
        user::{
            EditedUser, Profile, PublicProfile, User, UserCreatedUserWarning, UserMinimal,
            UserWarning,
        },
    },
    repositories::{
        conversation_repository::find_unread_conversations_amount,
        peer_repository,
        torrent_repository::search_torrents,
        user_repository::{
            create_user_warning, find_registered_users, find_user_profile, find_user_warnings,
            update_user,
        },
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUserQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/user",
    params(GetUserQuery),
    responses(
        (status = 200, description = "Successfully got the user's profile", body=PublicProfile),
    )
)]
pub async fn get_user(
    arc: web::Data<Arcadia>,
    query: web::Query<GetUserQuery>,
    current_user: User,
) -> Result<HttpResponse> {
    let user = find_user_profile(&arc.pool, &query.id).await?;

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
    let uploaded_torrents =
        search_torrents(&arc.pool, &torrent_search, Some(current_user.id)).await?;
    torrent_search.torrent.snatched_by_id = Some(query.id);
    torrent_search.torrent.created_by_id = None;
    torrent_search.sort_by = TorrentSearchSortField::TorrentSnatchedAt;
    let snatched_torrents =
        search_torrents(&arc.pool, &torrent_search, Some(current_user.id)).await?;

    Ok(HttpResponse::Created().json(json!({
        "user":user,
        "last_five_uploaded_torrents": uploaded_torrents.get("title_groups").unwrap(),
        "last_five_snatched_torrents": snatched_torrents.get("title_groups").unwrap()
    })))
}

#[utoipa::path(
    get,
    path = "/api/me",
    responses(
        (status = 200, description = "Successfully got the user's profile", body=Profile),
    )
)]
pub async fn get_me(mut current_user: User, arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    current_user.password_hash = String::from("");
    let peers = peer_repository::get_user_peers(&arc.pool, current_user.id).await;
    let user_warnings = find_user_warnings(&arc.pool, current_user.id).await;
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
    let uploaded_torrents =
        search_torrents(&arc.pool, &torrent_search, Some(current_user.id)).await?;
    torrent_search.torrent.snatched_by_id = Some(current_user.id);
    torrent_search.torrent.created_by_id = None;
    torrent_search.sort_by = TorrentSearchSortField::TorrentSnatchedAt;
    let snatched_torrents =
        search_torrents(&arc.pool, &torrent_search, Some(current_user.id)).await?;
    let unread_conversations_amount =
        find_unread_conversations_amount(&arc.pool, current_user.id).await?;
    Ok(HttpResponse::Ok().json(json!({
            "user": current_user,
            "peers":peers,
            "user_warnings": user_warnings,
            "unread_conversations_amount": unread_conversations_amount,
            "last_five_uploaded_torrents": uploaded_torrents.get("title_groups").unwrap(),
            "last_five_snatched_torrents": snatched_torrents.get("title_groups").unwrap()
    })))
}

#[utoipa::path(
    post,
    path = "/api/user/warn",
    responses(
        (status = 200, description = "Successfully warned the user", body=UserWarning),
    )
)]
pub async fn warn_user(
    form: web::Json<UserCreatedUserWarning>,
    current_user: User,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }
    let user_warning = create_user_warning(&arc.pool, current_user.id, &form).await?;

    Ok(HttpResponse::Created().json(user_warning))
}

#[utoipa::path(
    put,
    path = "/api/user",
    responses(
        (status = 200, description = "Successfully edited the user"),
    )
)]
pub async fn edit_user(
    form: web::Json<EditedUser>,
    current_user: User,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    update_user(&arc.pool, current_user.id, &form).await?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}

#[utoipa::path(
    get,
    path = "/api/registered-users",
    responses(
        (status = 200, description = "All registered users", body=Vec<UserMinimal>),
    )
)]
pub async fn get_registered_users(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    let users = find_registered_users(&arc.pool).await?;

    Ok(HttpResponse::Ok().json(users))
}
