use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        staff_pm::{StaffPmMessage, UserCreatedStaffPmMessage},
        user::UserClass,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
	post,
	operation_id = "Create staff PM message",
	tag = "StaffPM",
	path = "/api/staff-pms/messages",
	security(("http" = ["Bearer"])) ,
	responses((status = 201, description = "Created staff PM message", body = StaffPmMessage))
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    message: Json<UserCreatedStaffPmMessage>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let is_staff = user.class == UserClass::Staff;
    // Allow creator (non-staff) to reply only to their own thread, and staff to any. We'll rely on DB check in get step.
    // Quick check: ensure user has access to thread
    let _ = arc
        .pool
        .get_staff_pm(message.staff_pm_id, user.sub, is_staff)
        .await?;
    let created = arc.pool.create_staff_pm_message(&message, user.sub).await?;
    Ok(HttpResponse::Created().json(created))
}
