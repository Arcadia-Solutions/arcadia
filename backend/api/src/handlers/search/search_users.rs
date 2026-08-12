use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults,
        user::{SearchUsersQuery, UserPermission, UserSearchResult},
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
    current_user: Authdata,
    arc: Data<Arcadia<R>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(current_user.sub, &UserPermission::SearchUsers, req.path())
        .await?;

    let mut results = arc.pool.search_users(&query).await?;

    let can_see_paranoia_hidden_user_info = arc
        .pool
        .user_has_permission(current_user.sub, &UserPermission::SeeParanoiaHiddenUserInfo)
        .await?;
    if !can_see_paranoia_hidden_user_info {
        for user in &mut results.results {
            if user.id != current_user.sub {
                user.hide_paranoia_hidden_stats();
            }
        }
    }

    Ok(HttpResponse::Ok().json(results))
}
