use crate::{
    handlers::subscriptions::create_subscription_artist_title_groups::AddSubscriptionArtistTitleGroupsQuery,
    middlewares::auth_middleware::Authdata, Arcadia,
};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;

pub type RemoveSubscriptionArtistTitleGroupsQuery = AddSubscriptionArtistTitleGroupsQuery;

#[utoipa::path(
    delete,
    operation_id = "Remove artist title groups subscription",
    tag = "Subscription",
    path = "/api/subscriptions/artist-title-groups",
    params (RemoveSubscriptionArtistTitleGroupsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully unsubscribed to the item"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<RemoveSubscriptionArtistTitleGroupsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription_artist_title_groups(query.artist_id, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
