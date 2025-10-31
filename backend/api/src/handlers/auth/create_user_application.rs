use crate::Arcadia;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::user_application::{UserApplication, UserCreatedUserApplication},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create user application",
    tag = "User Application",
    path = "/api/auth/apply",
    responses(
        (status = 201, description = "Successfully created user application", body = UserApplication)
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    application: Json<UserCreatedUserApplication>,
) -> Result<HttpResponse> {
    let created_application = arc
        .pool
        .create_user_application(&application.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(created_application))
}
