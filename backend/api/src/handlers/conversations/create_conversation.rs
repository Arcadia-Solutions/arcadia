use crate::{
    handlers::User,
    Arcadia,
};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::conversation::{
    Conversation,
    UserCreatedConversation,
};

#[utoipa::path(
    post,
    path = "/api/conversation",
    responses(
        (status = 200, description = "Successfully created the conversation and first message", body=Conversation),
    )
)]
pub async fn exec(
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
