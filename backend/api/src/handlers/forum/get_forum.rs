use crate::Arcadia;
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::forum::ForumOverview, redis::RedisPoolInterface};
use serde_json::json;

#[utoipa::path(
    get,
    operation_id = "Create forum",
    tag = "Forum",
    path = "/api/forum",
    responses(
        (status = 200, description = "Returns an overview of the forum", body=ForumOverview),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let forum_overview = arc.pool.find_forum_overview().await?;
    let latest_forum_posts = arc.pool.find_latest_forum_posts(3).await?;

    Ok(HttpResponse::Ok().json(json!({
        "forum_overview": forum_overview,
        "latest_forum_posts": latest_forum_posts,
    })))
}
