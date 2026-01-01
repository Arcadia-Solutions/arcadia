use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::notification::NotificationForumThreadPost, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetNotificationsForumThreadPostsQuery {
    pub include_read: bool,
}

#[utoipa::path(
    get,
    operation_id = "Get notifications for forum thread posts",
    tag = "Notification",
    path = "/api/notifications/forum-thread-posts",
    params (GetNotificationsForumThreadPostsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the notifications", body = Vec<NotificationForumThreadPost>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetNotificationsForumThreadPostsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let notifications = arc
        .pool
        .find_notifications_forum_thread_posts(user.sub, query.include_read)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(notifications)))
}
