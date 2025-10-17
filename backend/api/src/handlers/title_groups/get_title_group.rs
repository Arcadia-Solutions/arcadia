use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_storage::{
    models::title_group::TitleGroupAndAssociatedData, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::Result;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTitleGroupQuery {
    pub id: i32,
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
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<GetTitleGroupQuery>,
    user: Authdata,
) -> Result<HttpResponse> {
    let title_group = arc
        .pool
        .find_title_group_hierarchy(query.id, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(title_group))
}
