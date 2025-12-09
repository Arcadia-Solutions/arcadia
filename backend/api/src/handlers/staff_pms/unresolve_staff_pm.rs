use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserClass, redis::RedisPoolInterface};

#[utoipa::path(
	put,
	operation_id = "Unresolve staff PM",
	tag = "StaffPM",
	path = "/api/staff-pms/{id}/unresolve",
	params(("id" = i64, Path, description = "Staff PM id")),
	security(("http" = ["Bearer"])) ,
	responses((status = 200, description = "Unresolved staff PM", body = arcadia_storage::models::staff_pm::StaffPm))
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    id: Path<i64>,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }
    let updated = arc.pool.unresolve_staff_pm(id.into_inner(), user.sub).await?;
    Ok(HttpResponse::Ok().json(updated))
}
