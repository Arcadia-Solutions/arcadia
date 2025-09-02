use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_storage::{
    models::{
        title_group::{EditedTitleGroup, TitleGroup},
        user::UserClass,
    },
    redis::RedisPoolInterface,
};

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
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

    if user.class == UserClass::Staff || title_group.created_by_id == user.sub {
        let updated_title_group = arc.pool.update_title_group(&form, title_group.id).await?;
        return Ok(HttpResponse::Ok().json(updated_title_group));
    }

    Err(Error::InsufficientPrivileges)
}
