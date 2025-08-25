use actix_web::{web, HttpResponse};
use arcadia_storage::models::title_group::TitleGroupLite;

use crate::{handlers::title_groups::get_title_group::GetTitleGroupQuery, Arcadia};
use arcadia_common::error::Result;

pub type GetTitleGroupLiteQuery = GetTitleGroupQuery;

#[utoipa::path(
    get,
    operation_id = "Get title group info lite",
    tag = "Title Group",
    path = "/api/title-groups/lite",
    params(GetTitleGroupLiteQuery),
    responses(
        (status = 200, description = "Successfully got the title_group (lite info)", body=TitleGroupLite),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupLiteQuery>,
) -> Result<HttpResponse> {
    let title_group = arc
        .pool
        .find_title_group_info_lite(Some(query.id), None, &None, 1)
        .await?;

    Ok(HttpResponse::Ok().json(title_group))
}
