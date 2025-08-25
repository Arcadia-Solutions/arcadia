use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::{
    ForumOverview, ForumPost, ForumSubCategoryHierarchy, ForumThread, ForumThreadHierarchy,
    UserCreatedForumPost, UserCreatedForumThread,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[utoipa::path(
    post,
    path = "/api/forum/post",
    responses(
        (status = 200, description = "Successfully created the forum post", body=ForumPost),
    )
)]
pub async fn add_forum_post(
    forum_post: web::Json<UserCreatedForumPost>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let forum_post = arc
        .pool
        .create_forum_post(&forum_post, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(forum_post))
}

#[utoipa::path(
    post,
    path = "/api/forum/thread",
    responses(
        (status = 200, description = "Successfully created the forum thread", body=ForumThread),
    )
)]
pub async fn add_forum_thread(
    mut forum_thread: web::Json<UserCreatedForumThread>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let forum_thread = arc
        .pool
        .create_forum_thread(&mut forum_thread, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(forum_thread))
}

#[utoipa::path(
    get,
    path = "/api/forum",
    responses(
        (status = 200, description = "Returns an overview of the forum", body=ForumOverview),
    )
)]
pub async fn get_forum(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let forum_overview = arc.pool.find_forum_overview().await?;

    Ok(HttpResponse::Ok().json(forum_overview))
}

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
pub async fn get_forum_sub_category_threads(
    arc: web::Data<Arcadia>,
    query: web::Query<GetForumSubCategoryThreadsQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let threads = arc.pool.find_forum_sub_category_threads(query.id).await?;

    Ok(HttpResponse::Ok().json(threads))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadQuery {
    pub title: String,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadQueryId {
    pub id: i64,
}

#[utoipa::path(
    get,
    path = "/api/forum/thread",
    params(GetForumThreadQueryId),
    responses(
        (status = 200, description = "Returns the thread and its posts", body=ForumThreadHierarchy)
    )
)]
pub async fn get_forum_thread(
    arc: web::Data<Arcadia>,
    query_id: web::Query<GetForumThreadQueryId>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes

    let thread = arc.pool.find_forum_thread(query_id.0.id).await?;

    Ok(HttpResponse::Ok().json(thread))
}

#[utoipa::path(
    get,
    path = "/api/search/forum/threads",
    params(GetForumThreadQuery),
    responses(
        (status = 200, description = "Returns the threads and its posts", body=Vec<ForumThreadHierarchy>)
    )
)]
pub async fn search_forum_thread(
    arc: web::Data<Arcadia>,
    query: web::Query<GetForumThreadQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes

    let offset = query.0.offset.unwrap_or(0);
    let limit = query.0.limit.unwrap_or(10);

    let threads = arc.pool.search_forum(query.0.title, offset, limit).await?;

    Ok(HttpResponse::Ok().json(threads))
}
