use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::conversation::{Conversation, UserCreatedConversation},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create conversation",
    tag = "Conversation",
    path = "/api/conversations",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the conversation and first message", body=Conversation),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    mut conversation: Json<UserCreatedConversation>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    // creates a conversation and the first message, empty conversations should not be allowed
    let conversation = arc
        .pool
        .create_conversation(&mut conversation, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(conversation))
}
