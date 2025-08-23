use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::ForumThreadHierarchy;
use serde::Deserialize;
use utoipa::IntoParams;

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
pub async fn exec(
    arc: web::Data<Arcadia>,
    query_id: web::Query<GetForumThreadQueryId>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes

    let thread = arc.pool.find_forum_thread(query_id.0.id).await?;

    Ok(HttpResponse::Ok().json(thread))
}
