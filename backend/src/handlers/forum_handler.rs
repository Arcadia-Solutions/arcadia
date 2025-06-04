use crate::{
    Arcadia, Result,
    models::{
        forum::{
            ForumOverview, ForumPost, ForumSubCategoryHierarchy, ForumThread, ForumThreadAndPosts,
            UserCreatedForumPost, UserCreatedForumThread,
        },
        user::User,
    },
    repositories::forum_repository::{
        create_forum_post, create_forum_thread, find_forum_overview,
        find_forum_sub_category_threads, find_forum_thread, query_forum_thread,
    },
};
use actix_web::{HttpResponse, web};
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
    let forum_post = create_forum_post(&arc.pool, &forum_post, current_user.id).await?;

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
    let forum_thread = create_forum_thread(&arc.pool, &mut forum_thread, current_user.id).await?;

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
    let forum_overview = find_forum_overview(&arc.pool).await?;

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
    let threads = find_forum_sub_category_threads(&arc.pool, query.id).await?;

    Ok(HttpResponse::Ok().json(threads))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadQuery {
    pub name: String,
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
    params(GetForumThreadQuery, GetForumThreadQueryId),
    responses(
        (status = 200, description = "Returns the threads and its posts", body=ForumThreadAndPosts),
    )
)]
pub async fn get_forum_thread(
    arc: web::Data<Arcadia>,
    query_id: Option<web::Query<GetForumThreadQueryId>>,
    query: Option<web::Query<GetForumThreadQuery>>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes

    let thread = match (query_id, query) {
        (Some(q), _) => find_forum_thread(&arc.pool, q.0.id).await.map(|v| vec![v]),
        (_, Some(q)) => {
            let offset = q.0.offset.unwrap_or(0);
            let limit = q.0.limit.unwrap_or(10);

            query_forum_thread(&arc.pool, q.0.name, offset, limit).await
        }
        _ => Err(crate::Error::InvalidUserIdOrTorrentId),
    }?;

    Ok(HttpResponse::Ok().json(thread))
}
