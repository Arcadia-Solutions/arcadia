use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{forum::EditedForumThread, user::UserClass},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit forum thread",
    tag = "Forum",
    path = "/api/forum/thread",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the forum thread"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    edited_forum_thread: Json<EditedForumThread>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let original_forum_thread = arc
        .pool
        .find_forum_thread(edited_forum_thread.id, user.sub)
        .await?;

    if original_forum_thread.created_by_id == user.sub || user.class == UserClass::Staff {
        let forum_thread = arc
            .pool
            .update_forum_thread(&edited_forum_thread, user.sub)
            .await?;
        Ok(HttpResponse::Ok().json(forum_thread))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}
