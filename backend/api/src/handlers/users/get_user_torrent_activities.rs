use crate::{
    handlers::users::get_user_torrent_activities_overview::SeedersPerTorrent,
    middlewares::auth_middleware::Authdata, Arcadia,
};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_periodic_tasks::env::formula_to_sql;
use arcadia_storage::{
    models::torrent_activity::GetTorrentActivitiesQuery, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUserTorrentActivitiesQuery {
    #[serde(flatten)]
    #[param(inline)]
    pub activities_query: GetTorrentActivitiesQuery,
    pub hours_seeding_per_day: u8,
    pub seeders_per_torrent: SeedersPerTorrent,
}

#[utoipa::path(
    get,
    operation_id = "Get user torrent activities",
    tag = "User",
    path = "/api/users/torrent-activities",
    params(GetUserTorrentActivitiesQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the user's torrent activities"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    query: Query<GetUserTorrentActivitiesQuery>,
) -> Result<HttpResponse> {
    let seeders_sql = query.seeders_per_torrent.to_seeders_sql();
    let formula_sql = formula_to_sql(&arc.bonus_points_formula, seeders_sql)
        .map_err(|e| arcadia_common::error::Error::BadRequest(e.to_string()))?;

    let task_interval = arc.seedtime_and_bonus_points_update_seconds;
    let ticks_per_day = (query.hours_seeding_per_day as i64 * 3600) / task_interval as i64;

    let results = arc
        .pool
        .get_torrent_activities(
            user.sub,
            &query.activities_query,
            &formula_sql,
            ticks_per_day,
        )
        .await?;

    Ok(HttpResponse::Ok().json(results))
}
