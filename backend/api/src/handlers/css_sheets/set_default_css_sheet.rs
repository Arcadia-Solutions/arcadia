use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserClass, redis::RedisPoolInterface};

#[utoipa::path(
    put,
    operation_id = "Set default CSS sheet",
    tag = "Css Sheet",
    path = "/api/css-sheets/{name}/default",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully changed the default CSS sheet"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    name: Path<String>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    arc.pool.set_default_css_sheet(&name).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "result": "success" })))
}
