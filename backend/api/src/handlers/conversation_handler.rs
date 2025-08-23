use crate::{
    handlers::{User, UserId},
    Arcadia,
};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::conversation::{
    Conversation, ConversationHierarchy, ConversationMessage, ConversationsOverview,
    UserCreatedConversation, UserCreatedConversationMessage,
};
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;

#[utoipa::path(
    post,
    path = "/api/conversation",
    responses(
        (status = 200, description = "Successfully created the conversation and first message", body=Conversation),
    )
)]
pub async fn add_conversation(
    mut conversation: web::Json<UserCreatedConversation>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // creates a conversation and the first message, empty conversations should not be allowed
    let conversation = arc
        .pool
        .create_conversation(&mut conversation, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(conversation))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetConversationQuery {
    id: i64,
}

#[utoipa::path(
    get,
    params(GetConversationQuery),
    path = "/api/conversation",
    responses(
        (status = 200, description = "Found the conversation and its messages", body=ConversationHierarchy),
    )
)]
pub async fn get_conversation(
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

#[utoipa::path(
    get,
    path = "/api/conversations",
    responses(
        (status = 200, description = "Found the conversations and some of their metadata", body=ConversationsOverview),
    )
)]
pub async fn get_user_conversations(
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let conversations = arc.pool.find_user_conversations(current_user_id.0).await?;

    Ok(HttpResponse::Ok().json(json!({"conversations": conversations})))
}

#[utoipa::path(
    post,
    path = "/api/conversation/message",
    responses(
        (status = 200, description = "Successfully created the conversation's message", body=ConversationMessage),
    )
)]
pub async fn add_conversation_message(
    message: web::Json<UserCreatedConversationMessage>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let message = arc
        .pool
        .create_conversation_message(&message, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(message))
}
