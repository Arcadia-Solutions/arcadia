use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        donation::{SearchDonationsQuery, SearchDonationsResponse},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search donations",
    tag = "Donation",
    path = "/api/donations",
    security(
        ("http" = ["Bearer"])
    ),
    params(SearchDonationsQuery),
    responses(
        (status = 200, description = "Successfully retrieved donations with aggregates", body=SearchDonationsResponse),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<SearchDonationsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::SearchDonation)
        .await?;

    let donations = arc.pool.search_donations(&query).await?;

    Ok(HttpResponse::Ok().json(donations))
}
