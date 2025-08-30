use crate::{
    handlers::subscriptions::create_subscription::AddSubscriptionQuery,
    middlewares::jwt_middleware::Authdata, Arcadia,
};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;

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
pub async fn exec(
    query: web::Query<RemoveSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription(query.item_id, &query.item, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
