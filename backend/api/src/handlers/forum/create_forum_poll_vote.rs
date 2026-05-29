use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        forum::{ForumPollHierarchy, UserCreatedForumPollVote},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create forum poll vote",
    tag = "Forum",
    path = "/api/forum/poll/vote",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 201, description = "Successfully voted on the forum poll", body=ForumPollHierarchy),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    forum_poll_vote: Json<UserCreatedForumPollVote>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::CreateForumPollVote, req.path())
        .await?;

    let forum_poll = arc
        .pool
        .create_forum_poll_vote(&forum_poll_vote, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(forum_poll))
}
