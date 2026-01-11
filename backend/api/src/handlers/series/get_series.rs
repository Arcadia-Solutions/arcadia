use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        common::OrderByDirection,
        series::SeriesAndTitleGroupHierarchyLite,
        torrent::{TorrentSearch, TorrentSearchOrderByColumn},
    },
    redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetSeriesQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get series",
    tag = "Series",
    path = "/api/series",
    params (GetSeriesQuery),
    responses(
        (status = 200, description = "Successfully got the series", body=SeriesAndTitleGroupHierarchyLite),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<GetSeriesQuery>,
) -> Result<HttpResponse> {
    let series = arc.pool.find_series(&query.id).await?;

    let search_form = TorrentSearch {
        series_id: Some(query.id),
        page: 1,
        page_size: i64::MAX,
        order_by_column: TorrentSearchOrderByColumn::TitleGroupOriginalReleaseDate,
        order_by_direction: OrderByDirection::Desc,
        title_group_include_empty_groups: true,
        title_group_name: None,
        title_group_content_type: Vec::new(),
        title_group_category: Vec::new(),
        edition_group_source: Vec::new(),
        torrent_video_resolution: Vec::new(),
        torrent_language: Vec::new(),
        torrent_reported: None,
        torrent_staff_checked: None,
        torrent_created_by_id: None,
        torrent_snatched_by_id: None,
        artist_id: None,
        collage_id: None,
    };
    let title_groups_in_series = arc.pool.search_torrents(&search_form, None).await?;

    Ok(HttpResponse::Ok().json(SeriesAndTitleGroupHierarchyLite {
        series,
        title_groups: title_groups_in_series.results,
    }))
}
