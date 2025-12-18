use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserClass, redis::RedisPoolInterface};

#[utoipa::path(
    delete,
    operation_id = "Delete donation",
    tag = "Donation",
    path = "/api/donations/{id}",
    params(
        ("id" = i64, Path, description = "Donation ID")
    ),
    security(("http" = ["Bearer"])),
    responses(
        (status = 204, description = "Successfully deleted donation"),
        (status = 403, description = "Forbidden - Only staff members can delete donations"),
        (status = 404, description = "Not Found - Donation not found")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    id: Path<i64>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    arc.pool.delete_donation(*id).await?;

    Ok(HttpResponse::NoContent().finish())
}
