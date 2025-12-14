use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        title_group_tag::{EditedTitleGroupTag, TitleGroupTag},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit title group tag",
    tag = "Title Group Tag",
    path = "/api/title-group-tags",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the title group tag", body=TitleGroupTag),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    tag: Json<EditedTitleGroupTag>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditTitleGroupTag)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_tag = arc.pool.update_title_group_tag(&tag).await?;

    Ok(HttpResponse::Ok().json(updated_tag))
}
