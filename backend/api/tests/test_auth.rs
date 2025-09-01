pub mod common;
pub mod mocks;

use actix_web::{
    http::{header::HeaderValue, StatusCode},
    test::{call_service, read_body, read_body_json, TestRequest},
};
use arcadia_api::{services::auth::InvalidationEntry, OpenSignups};
use arcadia_storage::{
    connection_pool::ConnectionPool, models::user::RefreshToken, redis::RedisInterface,
};
use mocks::mock_redis::MockRedisPool;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};

use crate::{
    common::{
        auth_header, call_and_read_body_json, create_test_app, create_test_app_and_login, Profile,
    },
    mocks::mock_redis::MockRedis,
};

#[derive(PartialEq, Debug, Serialize)]
struct RegisterRequest<'a> {
    username: &'a str,
    password: &'a str,
    password_verify: &'a str,
    email: &'a str,
}

#[derive(PartialEq, Debug, Deserialize)]
struct RegisterResponse {
    username: String,
    email: String,
    registered_from_ip: String,
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_open_registration(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Enabled,
        1.0,
        1.0,
    )
    .await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    let user = read_body_json::<RegisterResponse, _>(resp).await;

    assert_eq!(
        user,
        RegisterResponse {
            username: "test_user".into(),
            email: "test_email@testdomain.com".into(),
            // TODO: strip unnecessary /32 host postfix
            registered_from_ip: "10.10.4.88/32".into(),
        }
    );
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_duplicate_username_registration(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Enabled,
        1.0,
        1.0,
    )
    .await;

    // Register first user
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "duplicate_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Try to register second user with same username
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.89"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "duplicate_user",
            password: "different_password",
            password_verify: "different_password",
            email: "different_email@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    // Verify appropriate error response
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Check error message in response body
    let body = read_body(resp).await;
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error["error"], "username already exists");
}

#[sqlx::test(
    fixtures("with_test_user", "with_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_failures(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Disabled,
        1.0,
        1.0,
    )
    .await;

    // No key specified.  Should fail.
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    // No invitation key provided when closed registration - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    // Invalid key specified.  Should fail.
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=invalid")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    // Invalid invitation key - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );
}

#[sqlx::test(
    fixtures("with_test_user", "with_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Disabled,
        1.0,
        1.0,
    )
    .await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user2",
            password: "test_password2",
            password_verify: "test_password2",
            email: "newuser@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    let user = read_body_json::<RegisterResponse, _>(resp).await;

    assert_eq!(
        user,
        RegisterResponse {
            username: "test_user2".into(),
            email: "newuser@testdomain.com".into(),
            // TODO: strip unnecessary /32 host postfix
            registered_from_ip: "10.10.4.88/32".into(),
        }
    );

    // Try again with same key.  Should fail.
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user3",
            password: "test_password3",
            password_verify: "test_password3",
            email: "newuser2@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    // Invitation key already used - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures("with_test_user", "with_expired_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_expired_failure(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Disabled,
        1.0,
        1.0,
    )
    .await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user2",
            password: "test_password2",
            password_verify: "test_password2",
            email: "newuser@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    // Expired invitation key - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_authorized_endpoint_after_login(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(pool, MockRedisPool::default(), 1.0, 1.0).await;

    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/me")
        .to_request();

    #[derive(PartialEq, Deserialize)]
    struct User {
        username: String,
    }
    #[derive(PartialEq, Deserialize)]
    struct MeResponse {
        user: User,
    }

    let user = call_and_read_body_json::<MeResponse, _>(&service, req).await;

    assert_eq!(user.user.username, "test_user");
}

#[sqlx::test(
    fixtures("with_test_banned_user"),
    migrations = "../storage/migrations"
)]
async fn test_login_with_banned_user(pool: PgPool) {
    let service = create_test_app(
        Arc::new(ConnectionPool::with_pg_pool(pool)),
        MockRedisPool::default(),
        OpenSignups::Disabled,
        1.0,
        1.0,
    )
    .await;

    // Login first
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "test_user",
            "password": "test_password",
            "remember_me": true,
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_refresh_with_invalidated_token(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(Arc::clone(&pool), MockRedisPool::default(), 1.0, 1.0).await;

    // invalidate user tokens
    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/me")
        .insert_header(("authorization", format!("Bearer {}", user.token.clone())))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let profile = read_body_json::<Profile, _>(resp).await;

    tokio::time::sleep(Duration::from_secs(1)).await;
    let mut redis_conn = MockRedis::default();
    let entry = InvalidationEntry::new(profile.user.id);
    redis_conn
        .set(profile.user.id.to_string(), to_string(&entry).unwrap())
        .await
        .unwrap();

    let (service, _) = create_test_app_and_login(
        Arc::clone(&pool),
        MockRedisPool::with_conn(redis_conn),
        1.0,
        1.0,
    )
    .await;

    let payload = RefreshToken {
        refresh_token: user.refresh_token.clone(),
    };
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/refresh-token")
        .set_json(payload)
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
