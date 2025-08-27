use crate::{
    handlers::{subscriptions::create_subscription::AddSubscriptionQuery, User},
    Arcadia,
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
    responses(
        (status = 200, description = "Successfully unsubscribed to the item"),
    )
)]
pub async fn exec(
    query: web::Query<RemoveSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription(query.item_id, &query.item, current_user.id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
