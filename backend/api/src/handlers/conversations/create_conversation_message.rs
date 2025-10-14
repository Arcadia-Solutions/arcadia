use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::conversation::{ConversationMessage, UserCreatedConversationMessage},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create conversation message",
    tag = "Conversation",
    path = "/api/conversations/messages",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the conversation's message", body=ConversationMessage),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    message: Json<UserCreatedConversationMessage>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let message = arc
        .pool
        .create_conversation_message(&message, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(message))
}
