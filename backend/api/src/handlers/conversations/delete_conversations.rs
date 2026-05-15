use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
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
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    body: Json<DeleteConversationsRequest>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .delete_conversations(&body.conversation_ids, user.sub)
        .await?;

    Ok(HttpResponse::Ok().finish())
}