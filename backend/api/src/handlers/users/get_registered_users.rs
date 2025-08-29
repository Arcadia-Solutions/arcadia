use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserMinimal, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get registered users",
    tag = "User",
    path = "/api/users/registered",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "All registered users", body=Vec<UserMinimal>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    // TODO: change on extracker integration
    if user.class != "tracker" {
        return Err(Error::InsufficientPrivileges);
    };
    let users = arc.pool.find_registered_users().await?;

    Ok(HttpResponse::Ok().json(users))
}
