use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::models::common::PaginatedResults;
use arcadia_storage::models::subscription::SearchSubscriptionsQuery;
use arcadia_storage::models::title_group::TitleGroupHierarchyLite;
use arcadia_storage::redis::RedisPoolInterface;

#[utoipa::path(
    get,
    operation_id = "Get title group torrents subscriptions",
    tag = "Subscription",
    path = "/api/subscriptions/title-group-torrents",
    params(SearchSubscriptionsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully retrieved title group torrents subscriptions", body = PaginatedResults<TitleGroupHierarchyLite>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<SearchSubscriptionsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let results = arc
        .pool
        .find_subscription_title_group_torrents(user.sub, &query)
        .await?;

    Ok(HttpResponse::Ok().json(results))
}
