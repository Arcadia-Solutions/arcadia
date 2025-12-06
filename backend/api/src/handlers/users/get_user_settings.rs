use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::user::UserSettings, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get user settings",
    tag = "User",
    path = "/api/users/settings",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully retrieved user settings", body = UserSettings),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let settings = arc.pool.get_user_settings(user.sub).await?;
    Ok(HttpResponse::Ok().json(settings))
}
