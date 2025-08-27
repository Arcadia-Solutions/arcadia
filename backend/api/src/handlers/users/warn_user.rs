use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::user::{UserCreatedUserWarning, UserWarning};

#[utoipa::path(
    post,
    operation_id = "Warn users",
    tag = "User",
    path = "/api/users/warnings",
    responses(
        (status = 200, description = "Successfully warned the user", body=UserWarning),
    )
)]
pub async fn exec(
    form: web::Json<UserCreatedUserWarning>,
    current_user: User,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }
    let user_warning = arc.pool.create_user_warning(current_user.id, &form).await?;

    Ok(HttpResponse::Created().json(user_warning))
}
