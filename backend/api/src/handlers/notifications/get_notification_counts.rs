use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::notification::NotificationCounts, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get notification counts",
    tag = "Notification",
    path = "/api/notifications/counts",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got notification counts", body = NotificationCounts),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let counts = arc.pool.find_notification_counts(user.sub).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(counts)))
}
