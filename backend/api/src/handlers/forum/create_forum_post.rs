use crate::{middlewares::jwt_middleware::JwtAuthData, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::{ForumPost, UserCreatedForumPost};

#[utoipa::path(
    post,
    operation_id = "Create forum post",
    tag = "Forum",
    path = "/api/forum/post",
    responses(
        (status = 200, description = "Successfully created the forum post", body=ForumPost),
    )
)]
pub async fn exec(
    forum_post: web::Json<UserCreatedForumPost>,
    arc: web::Data<Arcadia>,
    user: JwtAuthData,
) -> Result<HttpResponse> {
    let forum_post = arc.pool.create_forum_post(&forum_post, user.sub).await?;

    Ok(HttpResponse::Created().json(forum_post))
}
