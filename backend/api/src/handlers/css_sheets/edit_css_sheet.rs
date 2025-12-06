use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::css_sheet::{CssSheet, EditedCssSheet},
    models::user::UserClass,
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit CSS sheet",
    tag = "Css Sheet",
    path = "/api/css-sheets",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the CSS sheet", body = CssSheet),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    css_sheet: Json<EditedCssSheet>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    let updated = arc.pool.update_css_sheet(&css_sheet).await?;
    Ok(HttpResponse::Ok().json(updated))
}
