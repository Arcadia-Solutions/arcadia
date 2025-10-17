use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::models::staff_pm::StaffPmHierarchy;
use arcadia_storage::{models::user::UserClass, redis::RedisPoolInterface};

#[utoipa::path(
	get,
	operation_id = "Get staff PM",
	tag = "StaffPM",
	path = "/api/staff-pms/{id}",
	params(("id" = i64, Path, description = "Staff PM id")),
	security(("http" = ["Bearer"])) ,
	responses((status = 200, description = "Staff PM conversation details", body = StaffPmHierarchy))
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    id: Path<i64>,
) -> Result<HttpResponse> {
    let is_staff = user.class == UserClass::Staff;
    let conv = arc
        .pool
        .get_staff_pm(id.into_inner(), user.sub, is_staff)
        .await?;
    Ok(HttpResponse::Ok().json(conv))
}
