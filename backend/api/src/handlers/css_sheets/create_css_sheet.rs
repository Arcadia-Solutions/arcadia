use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        css_sheet::{CssSheet, UserCreatedCssSheet},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create CSS sheet",
    tag = "Css Sheet",
    path = "/api/css-sheets",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 201, description = "Successfully created the CSS sheet", body = CssSheet),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    css_sheet: Json<UserCreatedCssSheet>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::CreateCssSheet)
        .await?;

    let created = arc.pool.create_css_sheet(&css_sheet, user.sub).await?;
    Ok(HttpResponse::Created().json(created))
}
