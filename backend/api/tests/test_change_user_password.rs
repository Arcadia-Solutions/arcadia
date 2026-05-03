pub mod common;
pub mod mocks;

use actix_web::{
    http::StatusCode,
    test::{call_service, TestRequest},
};
use arcadia_storage::{connection_pool::ConnectionPool, models::user::Login};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

const NEW_PASSWORD: &str = "BrandNewPassword123";

fn login_request(username: &str, password: &str) -> actix_http::Request {
    TestRequest::post()
        .uri("/api/auth/login")
        .set_json(Login {
            username: username.into(),
            password: password.into(),
            remember_me: true,
        })
        .to_request()
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_self_change_password_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = TestRequest::put()
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/password")
        .set_json(serde_json::json!({
            "current_password": "test_password",
            "new_password": NEW_PASSWORD,
            "new_password_verify": NEW_PASSWORD,
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = call_service(&service, login_request("user_basic", NEW_PASSWORD)).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = call_service(&service, login_request("user_basic", "test_password")).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_self_change_password_wrong_current_password(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = TestRequest::put()
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/password")
        .set_json(serde_json::json!({
            "current_password": "wrong_password",
            "new_password": NEW_PASSWORD,
            "new_password_verify": NEW_PASSWORD,
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let resp = call_service(&service, login_request("user_basic", "test_password")).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_self_change_password_mismatched_verification(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = TestRequest::put()
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/password")
        .set_json(serde_json::json!({
            "current_password": "test_password",
            "new_password": NEW_PASSWORD,
            "new_password_verify": "DifferentPassword123",
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_change_other_user_password_without_permission(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = TestRequest::put()
        .insert_header(auth_header(&user.token))
        .uri("/api/users/118/password")
        .set_json(serde_json::json!({
            "new_password": NEW_PASSWORD,
            "new_password_verify": NEW_PASSWORD,
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let resp = call_service(&service, login_request("user_perm_edit", "test_password")).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_change_other_user_password_with_permission(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, admin) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::ChangeUserPassword)
            .await;

    let req = TestRequest::put()
        .insert_header(auth_header(&admin.token))
        .uri("/api/users/100/password")
        .set_json(serde_json::json!({
            "new_password": NEW_PASSWORD,
            "new_password_verify": NEW_PASSWORD,
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = call_service(&service, login_request("user_basic", NEW_PASSWORD)).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
