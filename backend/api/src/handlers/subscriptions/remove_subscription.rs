use crate::{
    handlers::{subscriptions::create_subscription::AddSubscriptionQuery, User},
    Arcadia,
};
use actix_web::{
    web::{self, Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;

pub type RemoveSubscriptionQuery = AddSubscriptionQuery;

#[utoipa::path(
    delete,
    operation_id = "Remove subscription",
    tag = "Subscription",
    path = "/api/subscriptions",
    params (RemoveSubscriptionQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully unsubscribed to the item"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<RemoveSubscriptionQuery>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription(query.item_id, &query.item, current_user.id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
