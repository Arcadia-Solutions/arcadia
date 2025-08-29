use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::conversation::ConversationHierarchy, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetConversationQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get conversations",
    tag = "Conversation",
    params(GetConversationQuery),
    path = "/api/conversations",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Found the conversation and its messages", body=ConversationHierarchy),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetConversationQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let conversation_with_messages = arc.pool.find_conversation(query.id, user.sub, true).await?;

    Ok(HttpResponse::Ok().json(conversation_with_messages))
}
