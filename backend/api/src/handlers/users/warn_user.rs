use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
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
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::WarnUser, req.path())
        .await?;

    if form.ban {
        arc.pool
            .require_permission(user.sub, &UserPermission::BanUser, req.path())
            .await?;
        arc.auth.invalidate(form.user_id).await?;
    }
    let user_warning = arc.pool.create_user_warning(user.sub, &form).await?;

    Ok(HttpResponse::Created().json(user_warning))
}
