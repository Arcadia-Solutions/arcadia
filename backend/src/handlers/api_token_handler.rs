use actix_web::{HttpResponse, web};

use crate::{
    Arcadia, Result,
    models::{
        api_token::{APIToken, UserCreatedAPIToken},
        user::User,
    },
    repositories::api_token_repository::create_api_token,
};

#[utoipa::path(
    post,
    path = "/api/token",
    request_body(content = UserCreatedAPIToken, content_type = "application/json"),
    responses(
        (status = 201, description = "Successfully created the API token", body=APIToken),
    )
)]
pub async fn add_api_token(
    form: web::Json<UserCreatedAPIToken>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let created_api_token = create_api_token(&arc.pool, &form, current_user.id).await?;

    Ok(HttpResponse::Created().json(created_api_token))
}
