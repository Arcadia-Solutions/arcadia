use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{
    handlers::User,
    repositories::user_application_repository::{self, ApplicationFilter},
    models::user_application::UserApplicationStatus,
    Arcadia, Result, Error,
};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct ApplicationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<String>,
    pub checked: Option<bool>,
}

#[utoipa::path(
    post,
    path = "/api/user-application",
    responses(
        (status = 201, description = "Successfully created user application", body = crate::models::user_application::UserApplication)
    )
)]
pub async fn add_user_application(
    data: web::Data<Arcadia>,
    application: web::Json<crate::models::user_application::UserCreatedUserApplication>,
) -> Result<HttpResponse> {
    let created_application = user_application_repository::create_user_application(
        &data.pool,
        &application.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Created().json(created_application))
}

#[utoipa::path(
    get,
    path = "/api/user-application",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of applications to return (default: 50)"),
        ("offset" = Option<i64>, Query, description = "Number of applications to skip (default: 0)"),
        ("status" = Option<String>, Query, description = "Filter by application status: 'pending', 'accepted', or 'rejected'"),
        ("checked" = Option<bool>, Query, description = "Filter by checked status: true for checked (accepted/rejected), false for unchecked (pending)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved user applications", body = Vec<crate::models::user_application::UserApplication>),
        (status = 400, description = "Bad Request - Invalid status parameter"),
        (status = 403, description = "Forbidden - Only staff members can view user applications")
    )
)]
pub async fn get_user_applications(
    data: web::Data<Arcadia>,
    user: User,
    query: web::Query<ApplicationQuery>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if !user.is_staff() {
        return Err(Error::InsufficientPrivileges);
    }

    // Determine the filter based on query parameters
    let filter = if let Some(status_str) = &query.status {
        match status_str.to_lowercase().as_str() {
            "pending" => ApplicationFilter::Status(UserApplicationStatus::Pending),
            "accepted" => ApplicationFilter::Status(UserApplicationStatus::Accepted),
            "rejected" => ApplicationFilter::Status(UserApplicationStatus::Rejected),
            _ => return Err(Error::GenericDatabaseError(sqlx::Error::Protocol("Invalid status parameter. Must be 'pending', 'accepted', or 'rejected'".to_string()))),
        }
    } else if let Some(checked) = query.checked {
        if checked {
            ApplicationFilter::Checked
        } else {
            ApplicationFilter::Unchecked
        }
    } else {
        ApplicationFilter::All
    };

    let applications = user_application_repository::find_user_applications(
        &data.pool,
        query.limit,
        query.offset,
        filter,
    ).await?;

    Ok(HttpResponse::Ok().json(applications))
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UpdateApplicationStatusRequest {
    pub status: UserApplicationStatus,
    pub invitation_id: Option<i64>,
}

#[utoipa::path(
    put,
    path = "/api/user-application/{id}/status",
    params(
        ("id" = i64, Path, description = "User application ID")
    ),
    request_body = UpdateApplicationStatusRequest,
    responses(
        (status = 200, description = "Successfully updated user application status", body = crate::models::user_application::UserApplication),
        (status = 403, description = "Forbidden - Only staff members can update user applications"),
        (status = 404, description = "User application not found")
    )
)]
pub async fn update_user_application_status(
    data: web::Data<Arcadia>,
    user: User,
    path: web::Path<i64>,
    request: web::Json<UpdateApplicationStatusRequest>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if !user.is_staff() {
        return Err(Error::InsufficientPrivileges);
    }

    let application_id = path.into_inner();
    let request = request.into_inner();

    let updated_application = user_application_repository::update_user_application_status(
        &data.pool,
        application_id,
        request.status,
        request.invitation_id,
    ).await?;

    Ok(HttpResponse::Ok().json(updated_application))
}
