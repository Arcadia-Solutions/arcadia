use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;
use utoipa::OpenApi;

use crate::middleware::auth::AuthenticatedUser;

#[utoipa::path(
    delete,
    path = "/api/friendships/remove/{user_id}",
    params(
        ("user_id" = i64, Path, description = "ID of the user to remove from friends")
    ),
    responses(
        (status = 200, description = "Friendship removed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Friendship not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn remove_friendship(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let other_user_id = path.into_inner();
    
    // Don't allow removing friendship with self
    if user.id == other_user_id {
        return Ok(HttpResponse::BadRequest().json("Cannot remove friendship with yourself"));
    }

    pool.remove_friendship(user.id, other_user_id).await?;

    Ok(HttpResponse::Ok().json("Friendship removed successfully"))
}

#[derive(OpenApi)]
#[openapi(paths(remove_friendship))]
pub struct RemoveFriendshipApiDoc;