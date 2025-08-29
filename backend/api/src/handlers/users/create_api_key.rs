use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::user::{APIKey, UserCreatedAPIKey},
    redis::RedisPoolInterface,
};

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
pub async fn exec<R: RedisPoolInterface + 'static>(
    body: Json<UserCreatedAPIKey>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    let created_api_key = arc.pool.create_api_key(&body, current_user.id).await?;

    Ok(HttpResponse::Created().json(created_api_key))
}
