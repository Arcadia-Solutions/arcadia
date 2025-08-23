use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::ForumSubCategoryHierarchy;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumSubCategoryThreadsQuery {
    id: i32,
}

#[utoipa::path(
    get,
    params(GetForumSubCategoryThreadsQuery),
    path = "/api/forum/sub-category",
    responses(
        (status = 200, description = "Returns the threads in the forum sub-category", body=ForumSubCategoryHierarchy),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetForumSubCategoryThreadsQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let threads = arc.pool.find_forum_sub_category_threads(query.id).await?;

    Ok(HttpResponse::Ok().json(threads))
}
