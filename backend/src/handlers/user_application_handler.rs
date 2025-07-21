use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::{
    handlers::User,
    repositories::user_application_repository,
    Arcadia, Result, Error,
};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[utoipa::path(
    post,
    path = "/api/user-application",
    responses(
        (status = 201, description = "Successfully created user application", body = crate::models::user_application::UserApplication),
        (status = 401, description = "Unauthorized - Authentication required")
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
    path = "/api/user-applications",
    params(
        ("limit" = Option<i64>, Query, description = "Maximum number of applications to return (default: 50)"),
        ("offset" = Option<i64>, Query, description = "Number of applications to skip (default: 0)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved user applications", body = Vec<crate::models::user_application::UserApplication>),
        (status = 401, description = "Unauthorized - Authentication required"),
        (status = 403, description = "Forbidden - Only staff members can view user applications")
    )
)]
pub async fn get_user_applications(
    data: web::Data<Arcadia>,
    user: User,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse> {
    // Check if user is staff
    if !user.is_staff() {
        return Err(Error::InsufficientPrivileges);
    }

    let applications = user_application_repository::get_all_user_applications(
        &data.pool,
        query.limit,
        query.offset,
    ).await?;

    Ok(HttpResponse::Ok().json(applications))
}
