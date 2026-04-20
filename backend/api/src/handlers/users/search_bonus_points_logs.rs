use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        bonus_points_log::{BonusPointsLog, SearchBonusPointsLogsQuery},
        common::PaginatedResults,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search bonus points logs",
    tag = "User",
    path = "/api/users/bonus-points-logs",
    params(SearchBonusPointsLogsQuery),
    security(("http" = ["Bearer"])),
    responses(
        (status = 200, description = "Paginated bonus points logs for the current user", body = PaginatedResults<BonusPointsLog>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    query: Query<SearchBonusPointsLogsQuery>,
) -> Result<HttpResponse> {
    let results = arc.pool.search_bonus_points_logs(user.sub, &query).await?;

    Ok(HttpResponse::Ok().json(results))
}
