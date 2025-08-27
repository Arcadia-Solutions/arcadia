use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::conversation::{ConversationMessage, UserCreatedConversationMessage};

#[utoipa::path(
    post,
    operation_id = "Create conversation message",
    tag = "Conversation",
    path = "/api/conversations/messages",
    responses(
        (status = 200, description = "Successfully created the conversation's message", body=ConversationMessage),
    )
)]
pub async fn exec(
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
