use crate::{
    Arcadia, Error, Result,
    handlers::User,
    models::user_application::UserApplicationStatus,
    repositories::user_application_repository::{self},
};
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct GetUserApplicationsQuery {
    pub limit: Option<i64>,
    pub page: Option<i64>,
    pub status: Option<UserApplicationStatus>,
}

#[utoipa::path(
    post,
    path = "/api/apply",
    responses(
        (status = 201, description = "Successfully created user application", body = crate::models::user_application::UserApplication)
    )
)]
pub async fn add_user_application(
    data: web::Data<Arcadia>,
    application: web::Json<crate::models::user_application::UserCreatedUserApplication>,
) -> Result<HttpResponse> {
    let created_application =
        user_application_repository::create_user_application(&data.pool, &application.into_inner())
            .await?;

    Ok(HttpResponse::Created().json(created_application))
}

#[utoipa::path(
    get,
    path = "/api/user-application",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of applications to return (default: 50)"),
        ("page" = Option<i64>, Query, description = "Page (default: 1)"),
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
    query: web::Query<GetUserApplicationsQuery>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let applications = user_application_repository::find_user_applications(
        &data.pool,
        query.limit.unwrap_or(50),
        query.page.unwrap_or(1),
        query.status.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(applications))
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct UpdateUserApplication {
    pub status: UserApplicationStatus,
    pub user_application_id: i64,
}

#[utoipa::path(
    put,
    path = "/api/user-application",
    request_body = UpdateUserApplication,
    responses(
        (status = 200, description = "Successfully updated user application status", body = crate::models::user_application::UserApplication),
        (status = 403, description = "Forbidden - Only staff members can update user applications"),
        (status = 404, description = "User application not found")
    )
)]
pub async fn update_user_application_status(
    data: web::Data<Arcadia>,
    user: User,
    form: web::Json<UpdateUserApplication>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_application = user_application_repository::update_user_application_status(
        &data.pool,
        form.user_application_id,
        form.status.clone(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(updated_application))
}
