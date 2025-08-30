use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::user::{APIKey, UserCreatedAPIKey};

#[utoipa::path(
    post,
    operation_id = "Create API key",
    tag = "User",
    path = "/api/users/api-keys",
    request_body(content = UserCreatedAPIKey, content_type = "application/json"),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 201, description = "Successfully created the API key", body=APIKey),
    )
)]
pub async fn exec(
    form: web::Json<UserCreatedAPIKey>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let created_api_key = arc.pool.create_api_key(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(created_api_key))
}
