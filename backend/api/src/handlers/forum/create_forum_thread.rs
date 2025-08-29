use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::forum::{ForumThread, UserCreatedForumThread},
    redis::RedisPoolInterface,
};

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
pub async fn exec<R: RedisPoolInterface + 'static>(
    mut forum_thread: Json<UserCreatedForumThread>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    let forum_thread = arc
        .pool
        .create_forum_thread(&mut forum_thread, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(forum_thread))
}
