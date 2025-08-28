use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::friendship::FriendRequestWithUser,
};
use utoipa::OpenApi;

use crate::middleware::auth::AuthenticatedUser;

#[utoipa::path(
    get,
    path = "/api/friendships/requests/received",
    responses(
        (status = 200, description = "Pending friend requests received by the user", body = Vec<FriendRequestWithUser>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_pending_friend_requests(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let friend_requests = pool.get_pending_friend_requests(user.id).await?;

    Ok(HttpResponse::Ok().json(friend_requests))
}

#[utoipa::path(
    get,
    path = "/api/friendships/requests/sent",
    responses(
        (status = 200, description = "Friend requests sent by the user", body = Vec<FriendRequestWithUser>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_sent_friend_requests(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let friend_requests = pool.get_sent_friend_requests(user.id).await?;

    Ok(HttpResponse::Ok().json(friend_requests))
}

#[derive(OpenApi)]
#[openapi(paths(get_pending_friend_requests, get_sent_friend_requests))]
pub struct GetFriendRequestsApiDoc;