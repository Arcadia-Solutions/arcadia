use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user_application::{UserApplication, UserApplicationStatus},
    redis::RedisPoolInterface,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UpdateUserApplication {
    pub status: UserApplicationStatus,
    pub user_application_id: i64,
}

#[utoipa::path(
    put,
    operation_id = "Update user application status",
    tag = "User Application",
    path = "/api/user-applications",
    request_body = UpdateUserApplication,
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully updated user application status", body = UserApplication),
        (status = 403, description = "Forbidden - Only staff members can update user applications"),
        (status = 404, description = "User application not found")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: User,
    form: Json<UpdateUserApplication>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_application = arc
        .pool
        .update_user_application_status(form.user_application_id, form.status.clone())
        .await?;

    Ok(HttpResponse::Ok().json(updated_application))
}
