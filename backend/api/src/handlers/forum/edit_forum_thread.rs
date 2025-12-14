use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        forum::{EditedForumThread, ForumThreadEnriched},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit forum thread",
    tag = "Forum",
    path = "/api/forum/thread",
    responses(
        (status = 200, description = "Edits the thread's information", body=ForumThreadEnriched)
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    edited_forum_thread: Json<EditedForumThread>,
) -> Result<HttpResponse> {
    let original_thread = arc
        .pool
        .find_forum_thread(edited_forum_thread.id, user.sub)
        .await?;

    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditForumThread)
        .await?
        && original_thread.created_by_id != user.sub
    {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_thread = arc
        .pool
        .update_forum_thread(&edited_forum_thread, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(updated_thread))
}
