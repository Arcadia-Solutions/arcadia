use actix_web::{web, HttpResponse};

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent::{TorrentSearch, TorrentSearchResults};

// #[derive(Debug, Deserialize, ToSchema)]
// pub enum SearchPeriod {
//     #[serde(rename = "24 hours")]
//     TwentyFourHours,
//     #[serde(rename = "30 days")]
//     ThirtyDays,
//     #[serde(rename = "1 year")]
//     OneYear,
//     #[serde(rename = "all time")]
//     AllTime,
// }

#[utoipa::path(
    get,
    operation_id = "Search torrents",
    tag = "Search",
    path = "/api/search/torrents/lite",
    responses(
        (status = 200, description = "Title groups and their torrents found", body=TorrentSearchResults),
    )
)]
pub async fn exec(
    form: web::Json<TorrentSearch>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let search_results = arc.pool.search_torrents(&form, Some(user.sub)).await?;

    Ok(HttpResponse::Ok().json(search_results))
}
