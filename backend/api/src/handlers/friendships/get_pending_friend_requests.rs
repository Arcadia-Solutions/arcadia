use crate::handlers::UserId;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/api/friendships/requests/received",
    responses(
        (status = 200, description = "Pending friend requests retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn exec(pool: web::Data<ConnectionPool>, user: UserId) -> Result<HttpResponse> {
    let friend_requests = pool.get_pending_friend_requests(*user).await?;
    Ok(HttpResponse::Ok().json(friend_requests))
}

#[derive(OpenApi)]
#[openapi(paths(exec))]
pub struct GetPendingFriendRequestsApiDoc;
