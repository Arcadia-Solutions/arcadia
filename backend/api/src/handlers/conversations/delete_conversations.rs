use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::conversation::DeleteConversationsRequest, redis::RedisPoolInterface,
};

#[utoipa::path(
    delete,
    operation_id = "Delete conversations",
    tag = "Conversation",
    path = "/api/conversations",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Conversations soft-deleted successfully"),
        (status = 400, description = "No conversation IDs provided"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    body: Json<DeleteConversationsRequest>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if body.conversation_ids.is_empty() {
        return Err(Error::ConversationIdsEmpty);
    }

    arc.pool
        .delete_conversations(&body.conversation_ids, user.sub)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
