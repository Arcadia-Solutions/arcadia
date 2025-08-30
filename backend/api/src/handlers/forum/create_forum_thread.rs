use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::{ForumThread, UserCreatedForumThread};

#[utoipa::path(
    post,
    operation_id = "Create forum thread",
    tag = "Forum",
    path = "/api/forum/thread",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the forum thread", body=ForumThread),
    )
)]
pub async fn exec(
    mut forum_thread: web::Json<UserCreatedForumThread>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let forum_thread = arc
        .pool
        .create_forum_thread(&mut forum_thread, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(forum_thread))
}
