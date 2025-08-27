use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::user_application::{
    UserApplication, UserApplicationStatus, UserCreatedUserApplication,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct GetUserApplicationsQuery {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub status: Option<UserApplicationStatus>,
}

#[utoipa::path(
    post,
    operation_id = "Create user application",
    tag = "User Application",
    path = "/api/user-applications",
    responses(
        (status = 201, description = "Successfully created user application", body = UserApplication)
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    application: web::Json<UserCreatedUserApplication>,
) -> Result<HttpResponse> {
    let created_application = arc
        .pool
        .create_user_application(&application.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(created_application))
}
