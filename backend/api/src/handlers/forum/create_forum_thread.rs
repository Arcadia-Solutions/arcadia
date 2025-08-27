use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::forum::{ForumThread, UserCreatedForumThread};

#[utoipa::path(
    post,
    operation_id = "Create forum thread",
    tag = "Forum",
    path = "/api/forum/thread",
    responses(
        (status = 200, description = "Successfully created the forum thread", body=ForumThread),
    )
)]
pub async fn exec(
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
