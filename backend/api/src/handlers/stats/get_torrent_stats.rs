use crate::Arcadia;
use actix_web::{web::Data, web::Query, HttpRequest, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        torrent_stats::{TorrentStatsQuery, TorrentStatsResponse},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

use crate::middlewares::auth_middleware::Authdata;

#[utoipa::path(
    get,
    operation_id = "Get torrent stats",
    tag = "Stats",
    path = "/api/stats/torrents",
    params(TorrentStatsQuery),
    security(("http" = ["Bearer"])),
    responses(
        (status = 200, description = "Torrent stats", body = TorrentStatsResponse),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<TorrentStatsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::ViewStatsDetails, req.path())
        .await?;

    let response = arc.pool.get_torrent_stats(&query).await?;

    Ok(HttpResponse::Ok().json(response))
}
