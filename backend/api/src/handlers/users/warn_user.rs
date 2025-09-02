use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{UserClass, UserCreatedUserWarning, UserWarning},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Warn users",
    tag = "User",
    path = "/api/users/warnings",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully warned the user", body=UserWarning),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<UserCreatedUserWarning>,
    user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }
    let user_warning = arc.pool.create_user_warning(user.sub, &form).await?;

    if user_warning.ban {
        arc.auth.invalidate(user.sub).await?;
    }

    Ok(HttpResponse::Created().json(user_warning))
}
