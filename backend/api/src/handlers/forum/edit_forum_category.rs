use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        forum::{EditedForumCategory, ForumCategory},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit forum category",
    tag = "Forum",
    path = "/api/forum/category",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the forum category", body=ForumCategory),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    edited_category: Json<EditedForumCategory>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditForumCategory)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::EditForumCategory
        )));
    }

    let updated_category = arc.pool.update_forum_category(&edited_category).await?;

    Ok(HttpResponse::Ok().json(updated_category))
}
