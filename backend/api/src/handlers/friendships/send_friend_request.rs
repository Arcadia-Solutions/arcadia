use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::friendship::{FriendRequest, UserCreatedFriendRequest},
};
use utoipa::OpenApi;

use crate::middleware::auth::AuthenticatedUser;

#[utoipa::path(
    post,
    path = "/api/friendships/send",
    request_body = UserCreatedFriendRequest,
    responses(
        (status = 200, description = "Friend request sent successfully", body = FriendRequest),
        (status = 400, description = "Bad request - users already friends or request exists"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn send_friend_request(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
    friend_request: web::Json<UserCreatedFriendRequest>,
) -> Result<HttpResponse> {
    // Prevent sending friend request to self
    if user.id == friend_request.receiver_id {
        return Ok(HttpResponse::BadRequest().json("Cannot send friend request to yourself"));
    }

    let created_request = pool
        .send_friend_request(user.id, &friend_request)
        .await?;

    Ok(HttpResponse::Ok().json(created_request))
}

#[derive(OpenApi)]
#[openapi(paths(send_friend_request))]
pub struct SendFriendRequestApiDoc;