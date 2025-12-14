use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserPermission, redis::RedisPoolInterface};

#[utoipa::path(
	put,
	operation_id = "Resolve staff PM",
	tag = "StaffPM",
	path = "/api/staff-pms/{id}/resolve",
	params(("id" = i64, Path, description = "Staff PM id")),
	security(("http" = ["Bearer"])) ,
	responses((status = 200, description = "Resolved staff PM", body = arcadia_storage::models::staff_pm::StaffPm))
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    id: Path<i64>,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::ResolveStaffPm)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }
    let updated = arc.pool.resolve_staff_pm(id.into_inner(), user.sub).await?;
    Ok(HttpResponse::Ok().json(updated))
}
