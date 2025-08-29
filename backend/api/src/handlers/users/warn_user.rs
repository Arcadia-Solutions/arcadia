use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{UserCreatedUserWarning, UserWarning},
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
    current_user: User,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }
    let user_warning = arc.pool.create_user_warning(current_user.id, &form).await?;

    Ok(HttpResponse::Created().json(user_warning))
}
