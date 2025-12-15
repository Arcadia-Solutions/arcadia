use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::PaginatedResults, title_group::TitleGroupHierarchyLite, torrent::TorrentSearch,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    get,
    operation_id = "Get collage entries",
    tag = "Collages",
    path = "/api/collages/entries",
    params(TorrentSearch),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Collage entries", body=PaginatedResults<TitleGroupHierarchyLite>),
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
