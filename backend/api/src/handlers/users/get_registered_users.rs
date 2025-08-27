use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::user::UserMinimal;

#[utoipa::path(
    get,
    operation_id = "Get registered users",
    tag = "User",
    path = "/api/users/registered",
    responses(
        (status = 200, description = "All registered users", body=Vec<UserMinimal>),
    )
)]
pub async fn exec(arc: web::Data<Arcadia>, current_user: User) -> Result<HttpResponse> {
    // TODO: change on extracker integration
    if current_user.class != "tracker" {
        return Err(Error::InsufficientPrivileges);
    };
    let users = arc.pool.find_registered_users().await?;

    Ok(HttpResponse::Ok().json(users))
}
