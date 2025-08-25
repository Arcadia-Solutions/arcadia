use crate::{handlers::UserId, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::conversation::ConversationHierarchy;
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
    responses(
        (status = 200, description = "Found the conversation and its messages", body=ConversationHierarchy),
    )
)]
pub async fn exec(
    query: web::Query<GetConversationQuery>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let conversation_with_messages = arc
        .pool
        .find_conversation(query.id, current_user_id.0, true)
        .await?;

    Ok(HttpResponse::Ok().json(conversation_with_messages))
}
