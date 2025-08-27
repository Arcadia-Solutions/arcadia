use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct AddSubscriptionQuery {
    pub item_id: i64,
    pub item: String,
}

#[utoipa::path(
    post,
    operation_id = "Create subscription",
    tag = "Subscription",
    path = "/api/subscriptions",
    params (AddSubscriptionQuery),
    responses(
        (status = 200, description = "Successfully subscribed to the item"),
    )
)]
pub async fn exec(
    query: web::Query<AddSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    arc.pool
        .create_subscription(query.item_id, &query.item, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(serde_json::json!({"result": "success"})))
}
