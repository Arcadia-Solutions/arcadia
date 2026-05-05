use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults,
        conversation::{ConversationSearchQuery, ConversationSearchResult},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search conversations",
    tag = "Search",
    path = "/api/search/conversations",
    params(ConversationSearchQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Search conversations", body=PaginatedResults<ConversationSearchResult>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<ConversationSearchQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let can_read_all_conversations = arc
        .pool
        .user_has_permission(user.sub, &UserPermission::ReadAllConversations)
        .await?;

    let results = arc
        .pool
        .search_conversations(user.sub, &query, can_read_all_conversations)
        .await?;

    Ok(HttpResponse::Ok().json(results))
}
