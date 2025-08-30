use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
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
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Found the conversation and its messages", body=ConversationHierarchy),
    )
)]
pub async fn exec(
    query: web::Query<GetConversationQuery>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let conversation_with_messages = arc.pool.find_conversation(query.id, user.sub, true).await?;

    Ok(HttpResponse::Ok().json(conversation_with_messages))
}
