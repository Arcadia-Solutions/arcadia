use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::friendship::{FriendRequest, FriendRequestResponse},
};
use utoipa::OpenApi;

use crate::middleware::auth::AuthenticatedUser;

#[utoipa::path(
    post,
    path = "/api/friendships/respond",
    request_body = FriendRequestResponse,
    responses(
        (status = 200, description = "Friend request response processed successfully", body = FriendRequest),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Friend request not found or already processed")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn respond_to_friend_request(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
    response: web::Json<FriendRequestResponse>,
) -> Result<HttpResponse> {
    let updated_request = pool
        .respond_to_friend_request(user.id, response.friend_request_id, response.accept)
        .await?;

    Ok(HttpResponse::Ok().json(updated_request))
}

#[derive(OpenApi)]
#[openapi(paths(respond_to_friend_request))]
pub struct RespondToFriendRequestApiDoc;