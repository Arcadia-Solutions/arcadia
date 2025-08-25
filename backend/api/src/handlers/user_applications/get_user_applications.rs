use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::user_application::{UserApplication, UserApplicationStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct GetUserApplicationsQuery {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub status: Option<UserApplicationStatus>,
}

#[utoipa::path(
    get,
    operation_id = "Get user applications",
    tag = "User Application",
    path = "/api/user-applications",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of applications to return (default: 50)"),
        ("page" = Option<i64>, Query, description = "Page (default: 1)"),
        ("status" = Option<String>, Query, description = "Filter by application status: 'pending', 'accepted', or 'rejected'"),
        ("checked" = Option<bool>, Query, description = "Filter by checked status: true for checked (accepted/rejected), false for unchecked (pending)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved user applications", body = Vec<UserApplication>),
        (status = 400, description = "Bad Request - Invalid status parameter"),
        (status = 403, description = "Forbidden - Only staff members can view user applications")
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    user: User,
    query: web::Query<GetUserApplicationsQuery>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let applications = arc
        .pool
        .find_user_applications(
            query.limit.unwrap_or(50),
            query.page.unwrap_or(1),
            query.status.clone(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(applications))
}
