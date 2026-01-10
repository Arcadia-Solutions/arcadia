use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::notification::NotificationTitleGroupComment, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetNotificationsTitleGroupCommentsQuery {
    pub include_read: bool,
}

#[utoipa::path(
    get,
    operation_id = "Get notifications for title group comments",
    tag = "Notification",
    path = "/api/notifications/title-group-comments",
    params (GetNotificationsTitleGroupCommentsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the notifications", body = Vec<NotificationTitleGroupComment>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetNotificationsTitleGroupCommentsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let notifications = arc
        .pool
        .find_notifications_title_group_comments(user.sub, query.include_read)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(notifications)))
}
