use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{UserCreatedUserWarning, UserPermission, UserWarning},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Warn user",
    tag = "User",
    path = "/api/users/warn",
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
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::WarnUser)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::WarnUser
        )));
    }
    if form.ban {
        if !arc
            .pool
            .user_has_permission(user.sub, &UserPermission::BanUser)
            .await?
        {
            return Err(Error::InsufficientPermissions(format!(
                "{:?}",
                UserPermission::BanUser
            )));
        }
        arc.auth.invalidate(form.user_id).await?;
    }
    let user_warning = arc.pool.create_user_warning(user.sub, &form).await?;

    Ok(HttpResponse::Created().json(user_warning))
}
