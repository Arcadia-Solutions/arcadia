use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::staff_pm::StaffPmOverview;
use arcadia_storage::{models::user::UserClass, redis::RedisPoolInterface};

#[utoipa::path(
	get,
	operation_id = "List staff PMs",
	tag = "StaffPM",
	path = "/api/staff-pms",
	security(("http" = ["Bearer"])) ,
	responses((status = 200, description = "List staff PM conversations", body = [StaffPmOverview]))
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let is_staff = user.class == UserClass::Staff;
    let conversations = arc.pool.list_staff_pms(user.sub, is_staff).await?;
    Ok(HttpResponse::Ok().json(conversations))
}
