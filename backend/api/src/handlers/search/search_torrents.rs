use actix_web::{
    web::{Data, Query},
    HttpResponse,
};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults, title_group::TitleGroupHierarchyLite, torrent::TorrentSearch,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Search torrents",
    tag = "Search",
    params (TorrentSearch),
    path = "/api/search/torrents/lite",
    responses(
        (status = 200, description = "Title groups and their torrents found", body=PaginatedResults<TitleGroupHierarchyLite>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Query<TorrentSearch>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let search_results = arc.pool.search_torrents(&form, Some(user.sub)).await?;

    Ok(HttpResponse::Ok().json(search_results))
}
