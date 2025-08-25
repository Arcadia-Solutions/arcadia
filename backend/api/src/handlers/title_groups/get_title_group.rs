use actix_web::{web, HttpResponse};
use arcadia_storage::models::title_group::TitleGroupAndAssociatedData;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{handlers::User, Arcadia};
use arcadia_common::error::Result;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTitleGroupQuery {
    pub id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get title group",
    tag = "Title Group",
    path = "/api/title-groups",
    params(GetTitleGroupQuery),
    responses(
        (status = 200, description = "Successfully got the title_group", body=TitleGroupAndAssociatedData),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupQuery>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = arc
        .pool
        .find_title_group_hierarchy(query.id, &current_user)
        .await?;

    Ok(HttpResponse::Ok().json(title_group))
}
