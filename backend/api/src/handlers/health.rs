use actix_web::HttpResponse;

/// Unauthenticated health-check endpoint used by the Docker health check.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
