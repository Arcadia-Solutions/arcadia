use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults,
        unauthorized_access::{SearchUnauthorizedAccessQuery, UnauthorizedAccess},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search unauthorized access logs",
    tag = "Unauthorized Access",
    path = "/api/unauthorized-access",
    params(SearchUnauthorizedAccessQuery),
    responses(
        (status = 200, description = "Paginated list of unauthorized access logs", body = PaginatedResults<UnauthorizedAccess>),
        (status = 403, description = "Forbidden"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    query: Query<SearchUnauthorizedAccessQuery>,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::SearchUnauthorizedAccess)
        .await?;

    let results = arc
        .pool
        .find_unauthorized_accesses(query.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(results))
}
