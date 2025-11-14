use crate::{
    handlers::subscriptions::create_subscription_forum_thread_posts::AddSubscriptionForumThreadPostsQuery,
    middlewares::auth_middleware::Authdata, Arcadia,
};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;

pub type RemoveSubscriptionForumThreadPostsQuery = AddSubscriptionForumThreadPostsQuery;

#[utoipa::path(
    delete,
    operation_id = "Remove forum thread posts subscription",
    tag = "Subscription",
    path = "/api/subscriptions",
    params (RemoveSubscriptionForumThreadPostsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully unsubscribed to the item"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<RemoveSubscriptionForumThreadPostsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription_forum_thread_posts(query.thread_id, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
