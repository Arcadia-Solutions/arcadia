use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::notification::NotificationStaffPmMessage, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetNotificationsStaffPmMessagesQuery {
    pub include_read: bool,
}

#[utoipa::path(
    get,
    operation_id = "Get notifications for staff PM messages",
    tag = "Notification",
    path = "/api/notifications/staff-pm-messages",
    params (GetNotificationsStaffPmMessagesQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the notifications", body = Vec<NotificationStaffPmMessage>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetNotificationsStaffPmMessagesQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let notifications = arc
        .pool
        .find_notifications_staff_pm_messages(user.sub, query.include_read)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(notifications)))
}
