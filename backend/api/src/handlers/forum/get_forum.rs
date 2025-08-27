use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::ForumOverview;

#[utoipa::path(
    get,
    operation_id = "Create forum",
    tag = "Forum",
    path = "/api/forum",
    responses(
        (status = 200, description = "Returns an overview of the forum", body=ForumOverview),
    )
)]
pub async fn exec(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let forum_overview = arc.pool.find_forum_overview().await?;

    Ok(HttpResponse::Ok().json(forum_overview))
}
