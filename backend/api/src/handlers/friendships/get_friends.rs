use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{connection_pool::ConnectionPool, models::friendship::FriendshipWithUser};
use utoipa::OpenApi;

use crate::handlers::UserId;

#[utoipa::path(
    get,
    path = "/api/friendships/list",
    responses(
        (status = 200, description = "User's friends list", body = Vec<FriendshipWithUser>),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn exec(pool: web::Data<ConnectionPool>, user: UserId) -> Result<HttpResponse> {
    let friends = pool.get_user_friends(*user).await?;

    Ok(HttpResponse::Ok().json(friends))
}

#[derive(OpenApi)]
#[openapi(paths(exec))]
pub struct GetFriendsApiDoc;
