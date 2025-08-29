use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::conversation::ConversationsOverview, redis::RedisPoolInterface};
use serde_json::json;

#[utoipa::path(
    get,
    operation_id = "Get user conversations",
    tag = "User",
    path = "/api/users/conversations",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Found the conversations and some of their metadata", body=ConversationsOverview),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let conversations = arc.pool.find_user_conversations(user.sub).await?;

    Ok(HttpResponse::Ok().json(json!({"conversations": conversations})))
}
