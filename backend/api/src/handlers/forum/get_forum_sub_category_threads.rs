use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::forum::ForumSubCategoryHierarchy, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumSubCategoryThreadsQuery {
    id: i32,
}

#[utoipa::path(
    get,
    operation_id = "Get forim sub-category thread",
    tag = "Forum",
    path = "/api/forum/sub-category",
    params(GetForumSubCategoryThreadsQuery),
    responses(
        (status = 200, description = "Returns the threads in the forum sub-category", body=ForumSubCategoryHierarchy),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<GetForumSubCategoryThreadsQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let threads = arc.pool.find_forum_sub_category_threads(query.id).await?;

    Ok(HttpResponse::Ok().json(threads))
}
