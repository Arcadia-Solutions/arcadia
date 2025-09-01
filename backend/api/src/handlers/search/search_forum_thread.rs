use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::forum::ForumThreadHierarchy, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadQuery {
    pub title: String,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[utoipa::path(
    get,
    operation_id = "Search forum threads",
    tag = "Search",
    path = "/api/search/forum/threads",
    params(GetForumThreadQuery),
    responses(
        (status = 200, description = "Returns the threads and its posts", body=Vec<ForumThreadHierarchy>)
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    query: Query<GetForumThreadQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes

    let offset = query.0.offset.unwrap_or(0);
    let limit = query.0.limit.unwrap_or(10);

    let threads = arc.pool.search_forum(query.0.title, offset, limit).await?;

    Ok(HttpResponse::Ok().json(threads))
}
