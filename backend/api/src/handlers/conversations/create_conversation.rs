use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::conversation::{Conversation, UserCreatedConversation};

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
pub async fn exec(
    mut conversation: web::Json<UserCreatedConversation>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    // creates a conversation and the first message, empty conversations should not be allowed
    let conversation = arc
        .pool
        .create_conversation(&mut conversation, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(conversation))
}
