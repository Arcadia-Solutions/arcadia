use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        forum::{EditedForumPost, ForumPost},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit forum post",
    tag = "Forum",
    path = "/api/forum/post",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the forum post", body=ForumPost),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    edited_forum_post: Json<EditedForumPost>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let original_forum_post = arc.pool.find_forum_post(edited_forum_post.id).await?;

    if original_forum_post.created_by_id == user.sub
        || arc
            .pool
            .user_has_permission(user.sub, &UserPermission::EditForumPost)
            .await?
    {
        let forum_post = arc.pool.update_forum_post(&edited_forum_post).await?;
        Ok(HttpResponse::Created().json(forum_post))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}
