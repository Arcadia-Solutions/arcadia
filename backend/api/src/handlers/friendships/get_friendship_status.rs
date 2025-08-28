use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::friendship::FriendshipStatus,
};
use utoipa::OpenApi;

use crate::middleware::auth::AuthenticatedUser;

#[utoipa::path(
    get,
    path = "/api/friendships/status/{user_id}",
    params(
        ("user_id" = i64, Path, description = "ID of the user to check friendship status with")
    ),
    responses(
        (status = 200, description = "Friendship status between users", body = FriendshipStatus),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_friendship_status(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let other_user_id = path.into_inner();
    
    // Don't allow checking friendship status with self
    if user.id == other_user_id {
        return Ok(HttpResponse::BadRequest().json("Cannot check friendship status with yourself"));
    }

    let friendship_status = pool.get_friendship_status(user.id, other_user_id).await?;

    Ok(HttpResponse::Ok().json(friendship_status))
}

#[derive(OpenApi)]
#[openapi(paths(get_friendship_status))]
pub struct GetFriendshipStatusApiDoc;