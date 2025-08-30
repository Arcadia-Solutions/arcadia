use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;
use utoipa::OpenApi;

use crate::handlers::UserId;

#[utoipa::path(
    delete,
    path = "/api/friendships/cancel/{request_id}",
    params(
        ("request_id" = i64, Path, description = "ID of the friend request to cancel")
    ),
    responses(
        (status = 200, description = "Friend request cancelled successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Friend request not found or cannot be cancelled")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn exec(
    pool: web::Data<ConnectionPool>,
    user: UserId,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let request_id = path.into_inner();

    pool.cancel_friend_request(*user, request_id).await?;

    Ok(HttpResponse::Ok().json("Friend request cancelled successfully"))
}

#[derive(OpenApi)]
#[openapi(paths(exec))]
pub struct CancelFriendRequestApiDoc;
