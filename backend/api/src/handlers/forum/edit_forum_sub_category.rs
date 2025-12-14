use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        forum::{EditedForumSubCategory, ForumSubCategory},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit forum sub-category",
    tag = "Forum",
    path = "/api/forum/sub-category",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the forum sub-category", body=ForumSubCategory),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    edited_sub_category: Json<EditedForumSubCategory>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditForumSubCategory)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_sub_category = arc
        .pool
        .update_forum_sub_category(&edited_sub_category)
        .await?;

    Ok(HttpResponse::Ok().json(updated_sub_category))
}
