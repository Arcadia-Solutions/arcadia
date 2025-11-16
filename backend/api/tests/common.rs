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
use arcadia_storage::models::user::Login;
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::user::{LoginResponse, User},
    redis::RedisPoolInterface,
};
use envconfig::Envconfig;
use serde::{de::DeserializeOwned, Deserialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Profile {
    pub user: User,
}

pub async fn create_test_app<R: RedisPoolInterface + 'static>(
    pool: Arc<ConnectionPool>,
    redis_pool: R,
    open_signups: OpenSignups,
    global_upload_factor: i16,
    global_download_factor: i16,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let mut env = Env::init_from_env().unwrap();
    env.open_signups = open_signups;
    env.global_upload_factor = global_upload_factor;
    env.global_download_factor = global_download_factor;

    let arc = Arcadia::<R>::new(pool, Arc::new(redis_pool), env);

    // TODO: CORS?
    test::init_service(
        App::new()
            .app_data(web::Data::new(arc))
            .configure(arcadia_api::routes::init::<R>),
    )
    .await
}

pub enum TestUser {
    // Requires the "with_test_user" fixture.
    Standard,

    // Requires the "with_test_user2" fixture.
    Staff,
}

impl TestUser {
    fn get_login_payload(&self) -> Login {
        match self {
            TestUser::Standard => Login {
                username: "test_user".into(),
                password: "test_password".into(),
                remember_me: true,
            },
            TestUser::Staff => Login {
                username: "test_user2".into(),
                password: "test_password".into(),
                remember_me: true,
            },
        }
    }
}

// Requires "with_test_user" fixture.
pub async fn create_test_app_and_login<R: RedisPoolInterface + 'static>(
    pool: Arc<ConnectionPool>,
    redis_pool: R,
    global_upload_factor: i16,
    global_download_factor: i16,
    test_user: TestUser,
) -> (
    impl Service<Request, Response = ServiceResponse, Error = Error>,
    LoginResponse,
) {
    let service = create_test_app(
        pool,
        redis_pool,
        OpenSignups::Disabled,
        global_upload_factor,
        global_download_factor,
    )
    .await;

    // Login first
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(test_user.get_login_payload())
        .to_request();

    let user = call_and_read_body_json::<LoginResponse, _>(&service, req).await;

    assert!(!user.token.is_empty());
    assert!(!user.refresh_token.is_empty());

    (service, user)
}

pub fn auth_header(token: &str) -> impl TryIntoHeaderPair {
    (AUTHORIZATION, format!("Bearer {}", token))
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
