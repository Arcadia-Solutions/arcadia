use actix_web::{web, HttpResponse};
use arcadia_storage::models::title_group::TitleGroupAndAssociatedData;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
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
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the title_group", body=TitleGroupAndAssociatedData),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupQuery>,
    user: Authdata,
) -> Result<HttpResponse> {
    let title_group = arc
        .pool
        .find_title_group_hierarchy(query.id, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(title_group))
}
