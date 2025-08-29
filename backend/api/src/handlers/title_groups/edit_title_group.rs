use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_storage::{
    models::title_group::{EditedTitleGroup, TitleGroup},
    redis::RedisPoolInterface,
};

use crate::{handlers::User, Arcadia};
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
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = arc.pool.find_title_group(form.id).await?;

    if title_group.created_by_id == current_user.id || current_user.class == "staff" {
        let updated_title_group = arc.pool.update_title_group(&form, title_group.id).await?;
        Ok(HttpResponse::Ok().json(updated_title_group))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}
