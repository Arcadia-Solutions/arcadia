use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults,
        user::{SearchUsersQuery, UserSearchResult},
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search users",
    tag = "Search",
    path = "/api/search/users",
    params (SearchUsersQuery),
    description = "Search registered users with pagination. Case insensitive username search.",
    responses(
        (status = 200, description = "Successfully searched users", body=PaginatedResults<UserSearchResult>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<SearchUsersQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let results = arc.pool.search_users(&query).await?;

    Ok(HttpResponse::Ok().json(results))
}
