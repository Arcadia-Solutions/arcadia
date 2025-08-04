use actix_web::{HttpResponse, web};

use crate::{
    models::{
        api_key::{APIKey, UserCreatedAPIKey},
        user::User,
    }, repositories::auth_repository::create_api_key, Arcadia, Result
};

#[utoipa::path(
    post,
    path = "/api/api-key",
    request_body(content = UserCreatedAPIKey, content_type = "application/json"),
    responses(
        (status = 201, description = "Successfully created the API key", body=APIKey),
    )
)]
pub async fn add_api_key(
    form: web::Json<UserCreatedAPIKey>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let created_api_key = create_api_key(&arc.pool, &form, current_user.id).await?;

    Ok(HttpResponse::Created().json(created_api_key))
}
