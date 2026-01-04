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
use arcadia_api::{env::Env, Arcadia};
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
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let env = Env::init_from_env().unwrap();

    // Load settings from database for tests
    let settings = pool
        .get_arcadia_settings()
        .await
        .expect("failed to load arcadia settings from database");

    let arc = Arcadia::<R>::new(pool, Arc::new(redis_pool), env, settings);

    // TODO: CORS?
    test::init_service(
        App::new()
            .app_data(web::Data::new(arc))
            .configure(arcadia_api::routes::init::<R>),
    )
    .await
}

pub enum TestUser {
    // Requires the "with_test_users" fixture.
    Standard,
    EditArtist,
    EditSeries,
    EditTitleGroupComment,
    CreateCssSheet,
    EditCssSheet,
    CreateForumCategory,
    EditForumCategory,
    CreateForumSubCategory,
    EditForumSubCategory,
    EditForumThread,
    EditForumPost,
    DeleteForumCategory,
    DeleteForumSubCategory,
    DeleteForumThread,
    DeleteForumPost,
    ForumCategoryFlow,
    ForumSubCategoryFlow,
    CreateUserClass,
    EditUserClass,
    DeleteUserClass,
    EditUserPermissions,
    LockUserClass,
    ChangeUserClass,
    EditArcadiaSettings,
    CreateDonation,
    EditDonation,
    DeleteDonation,
    SearchDonation,
    WarnBanUser,
    SearchUnauthorizedAccess,
    DeleteArtist,
    SetTorrentStaffChecked,
    SearchUserEditChangeLogs,
}

impl TestUser {
    fn get_login_payload(&self) -> Login {
        let username = match self {
            TestUser::Standard => "user_basic",
            TestUser::EditArtist => "user_edit_art",
            TestUser::EditSeries => "user_edit_ser",
            TestUser::EditTitleGroupComment => "user_edit_tgc",
            TestUser::CreateCssSheet => "user_css_crt",
            TestUser::EditCssSheet => "user_css_edit",
            TestUser::CreateForumCategory => "user_cat_crt",
            TestUser::EditForumCategory => "user_cat_edit",
            TestUser::CreateForumSubCategory => "user_sub_crt",
            TestUser::EditForumSubCategory => "user_sub_edit",
            TestUser::EditForumThread => "user_thr_edit",
            TestUser::EditForumPost => "user_post_edit",
            TestUser::DeleteForumCategory => "user_cat_del",
            TestUser::DeleteForumSubCategory => "user_sub_del",
            TestUser::DeleteForumThread => "user_thr_del",
            TestUser::DeleteForumPost => "user_post_del",
            TestUser::ForumCategoryFlow => "user_cat_flow",
            TestUser::ForumSubCategoryFlow => "user_sub_flow",
            TestUser::CreateUserClass => "user_cls_crt",
            TestUser::EditUserClass => "user_cls_edit",
            TestUser::DeleteUserClass => "user_cls_del",
            TestUser::EditUserPermissions => "user_perm_edit",
            TestUser::LockUserClass => "user_lock_cls",
            TestUser::ChangeUserClass => "user_cls_chg",
            TestUser::EditArcadiaSettings => "user_arc_set",
            TestUser::CreateDonation => "user_don_crt",
            TestUser::EditDonation => "user_don_edit",
            TestUser::DeleteDonation => "user_don_del",
            TestUser::SearchDonation => "user_don_srch",
            TestUser::WarnBanUser => "user_warn_ban",
            TestUser::SearchUnauthorizedAccess => "user_unauth",
            TestUser::DeleteArtist => "user_art_del",
            TestUser::SetTorrentStaffChecked => "user_tor_stfc",
            TestUser::SearchUserEditChangeLogs => "user_edit_log",
        };

        Login {
            username: username.into(),
            password: "test_password".into(),
            remember_me: true,
        }
    }
}

// Requires "with_test_users" fixture.
pub async fn create_test_app_and_login<R: RedisPoolInterface + 'static>(
    pool: Arc<ConnectionPool>,
    redis_pool: R,
    test_user: TestUser,
) -> (
    impl Service<Request, Response = ServiceResponse, Error = Error>,
    LoginResponse,
) {
    let service = create_test_app(pool, redis_pool).await;

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
