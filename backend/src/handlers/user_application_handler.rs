use actix_web::{web, HttpResponse};
use crate::{
    handlers::User,
    repositories::user_application_repository,
    Arcadia, Result,
};

#[utoipa::path(
    post,
    path = "/api/user-application",
    responses(
        (status = 201, description = "Successfully created user application", body = crate::models::user_application::UserApplication),
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
    responses(
        (status = 200, description = "Successfully retrieved user applications", body = Vec<crate::models::user_application::UserApplication>),
        (status = 403, description = "Forbidden - Only staff members can view user applications")
    )
)]
pub async fn get_user_applications(
    data: web::Data<Arcadia>,
    user: User,
) -> Result<HttpResponse> {
    // Check if user is staff
    if !user.is_staff() {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Only staff members can view user applications"
        })));
    }

    let applications = user_application_repository::get_all_user_applications(&data.pool).await?;

    Ok(HttpResponse::Ok().json(applications))
}
