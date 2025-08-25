use actix_http::Request;
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    http::{
        header::{HeaderValue, TryIntoHeaderPair, AUTHORIZATION, CONTENT_TYPE},
        StatusCode,
    },
    test, web, App, Error,
};
use arcadia_api::{env::Env, Arcadia, OpenSignups};
use arcadia_storage::{connection_pool::ConnectionPool, models::user::LoginResponse};
use envconfig::Envconfig;
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn create_test_app(
    pool: PgPool,
    open_signups: OpenSignups,
    global_upload_factor: f64,
    global_download_factor: f64,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let mut env = Env::init_from_env().unwrap();
    env.open_signups = open_signups;
    env.tracker.global_upload_factor = global_upload_factor;
    env.tracker.global_download_factor = global_download_factor;
    let arc = Arcadia::new(Arc::new(ConnectionPool::with_pg_pool(pool)), env);

    // TODO: CORS?
    test::init_service(
        App::new()
            .app_data(web::Data::new(arc))
            .configure(arcadia_api::routes::init),
    )
    .await
}

// Requires "with_test_user" fixture.
pub async fn create_test_app_and_login(
    pool: PgPool,
    global_upload_factor: f64,
    global_download_factor: f64,
) -> (
    impl Service<Request, Response = ServiceResponse, Error = Error>,
    impl TryIntoHeaderPair,
) {
    let service = create_test_app(
        pool,
        OpenSignups::Disabled,
        global_upload_factor,
        global_download_factor,
    )
    .await;

    // Login first
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "test_user",
            "password": "test_password",
            "remember_me": true,
        }))
        .to_request();

    let user = call_and_read_body_json::<LoginResponse, _>(&service, req).await;

    assert!(!user.token.is_empty());
    assert!(!user.refresh_token.is_empty());

    (service, (AUTHORIZATION, format!("Bearer {}", user.token)))
}

pub async fn read_body_bencode<T: DeserializeOwned, B: MessageBody>(
    resp: ServiceResponse<B>,
) -> Result<T, serde_bencode::Error> {
    let body = test::read_body(resp).await;
    serde_bencode::from_bytes(&body)
}

pub async fn call_and_read_body_json_with_status<T, S>(
    service: &S,
    req: Request,
    status_code: StatusCode,
) -> T
where
    S: Service<Request, Response = ServiceResponse, Error = Error>,
    T: DeserializeOwned,
{
    let resp = test::call_service(&service, req).await;

    assert_eq!(
        resp.status(),
        status_code,
        "expected HTTP status {}, got {}",
        status_code,
        resp.status()
    );

    let content_type = resp.headers().get(CONTENT_TYPE);

    assert_eq!(
        content_type,
        Some(&HeaderValue::from_static("application/json")),
        "expected Content-Type: application/json, got {content_type:?}"
    );

    test::read_body_json::<T, _>(resp).await
}

#[inline]
pub async fn call_and_read_body_json<T, S>(service: &S, req: Request) -> T
where
    S: Service<Request, Response = ServiceResponse, Error = Error>,
    T: DeserializeOwned,
{
    call_and_read_body_json_with_status(service, req, StatusCode::OK).await
}
