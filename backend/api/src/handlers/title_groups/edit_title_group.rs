use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_storage::{
    models::{
        title_group::{EditedTitleGroup, TitleGroup},
        user::UserPermission,
        user_edit_change_log::NewUserEditChangeLog,
    },
    redis::RedisPoolInterface,
};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};

#[utoipa::path(
    put,
    operation_id = "Edit title group",
    tag = "Title Group",
    path = "/api/title-groups",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the title group", body=TitleGroup),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditedTitleGroup>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let title_group = arc.pool.find_title_group(form.id).await?;

    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditTitleGroup)
        .await?
        && title_group.created_by_id != user.sub
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::EditTitleGroup
        )));
    }

    if let Some(edits) = title_group.diff(&form) {
        arc.pool
            .create_user_edit_change_log(&NewUserEditChangeLog {
                item_type: "title_group".to_string(),
                item_id: title_group.id as i64,
                edited_by_id: user.sub,
                edits,
            })
            .await?;
    }

    let updated_title_group = arc.pool.update_title_group(&form, title_group.id).await?;
    Ok(HttpResponse::Ok().json(updated_title_group))
}
