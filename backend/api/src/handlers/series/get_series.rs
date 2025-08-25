use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::series::SeriesAndTitleGroupHierarchyLite;
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
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetSeriesQuery>,
) -> Result<HttpResponse> {
    let series = arc.pool.find_series(&query.id).await?;

    Ok(HttpResponse::Ok().json(series))
}
