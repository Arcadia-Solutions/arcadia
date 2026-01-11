use actix_web::{web::Data, HttpRequest, HttpResponse};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
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
    req: HttpRequest,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let qs_config = serde_qs::Config::new(5, false);
    let form: TorrentSearch = qs_config
        .deserialize_str(req.query_string())
        .map_err(|e| Error::BadRequest(e.to_string()))?;
    let search_results = arc.pool.search_torrents(&form, Some(user.sub)).await?;

    Ok(HttpResponse::Ok().json(search_results))
}
