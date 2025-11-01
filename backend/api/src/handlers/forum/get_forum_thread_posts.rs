use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{common::PaginatedResults, forum::ForumPostHierarchy},
    redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadQuery {
    pub title: String,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadPostsQuery {
    pub thread_id: i64,
    pub page: u32,
    pub page_size: u32,
}

#[utoipa::path(
    get,
    operation_id = "Get forum thread's posts",
    tag = "Forum",
    path = "/api/forum/thread/posts",
    params(GetForumThreadPostsQuery),
    responses(
        (status = 200, description = "Returns the thread's posts", body=PaginatedResults<ForumPostHierarchy>)
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<GetForumThreadPostsQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes

    let thread = arc
        .pool
        .find_forum_thread_posts(query.thread_id, query.page, query.page_size)
        .await?;

    Ok(HttpResponse::Ok().json(thread))
}
