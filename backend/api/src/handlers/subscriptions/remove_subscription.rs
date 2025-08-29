use crate::{
    handlers::subscriptions::create_subscription::AddSubscriptionQuery,
    middlewares::jwt_middleware::Authdata, Arcadia,
};
use actix_web::{
    web::{Data, Query},
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
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription(query.item_id, &query.item, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
