use crate::Arcadia;
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::user_application::{
        UserApplication, UserApplicationStatus, UserCreatedUserApplication,
        UserCreatedUserApplicationRequest,
    },
    redis::RedisPoolInterface,
    sqlx::types::ipnetwork::IpNetwork,
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
    application: Json<UserCreatedUserApplicationRequest>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    let application_data = UserCreatedUserApplication {
        body: application.body.clone(),
        email: application.email.clone(),
        referral: application.referral.clone(),
        ip_address: client_ip,
    };

    let created_application = arc.pool.create_user_application(&application_data).await?;

    Ok(HttpResponse::Created().json(created_application))
}
