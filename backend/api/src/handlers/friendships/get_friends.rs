use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::friendship::FriendshipWithUser,
};
use utoipa::OpenApi;

use crate::middleware::auth::AuthenticatedUser;

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
pub async fn get_user_friends(
    pool: web::Data<ConnectionPool>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let friends = pool.get_user_friends(user.id).await?;

    Ok(HttpResponse::Ok().json(friends))
}

#[derive(OpenApi)]
#[openapi(paths(get_user_friends))]
pub struct GetFriendsApiDoc;