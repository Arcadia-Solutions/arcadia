use actix_web::{web, HttpResponse};
use arcadia_storage::models::title_group::{ContentType, TitleGroupLite};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::Arcadia;
use arcadia_common::error::Result;

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchTitleGroupLiteQuery {
    name: String,
    content_type: Option<ContentType>,
}

#[utoipa::path(
    get,
    path = "/api/search/title-groups/lite",
    params(SearchTitleGroupLiteQuery),
    responses(
        (status = 200, description = "Returns title groups with their name containing the provided string, only the 5 first matches", body=Vec<TitleGroupLite>),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<SearchTitleGroupLiteQuery>,
) -> Result<HttpResponse> {
    let title_groups = arc
        .pool
        .find_title_group_info_lite(None, Some(&query.name), &query.content_type, 5)
        .await?;

    Ok(HttpResponse::Ok().json(title_groups))
}
