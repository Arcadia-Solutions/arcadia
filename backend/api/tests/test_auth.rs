use actix_web::{
    http::{header::HeaderValue, StatusCode},
    test,
};
use arcadia_api::OpenSignups;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod common;

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
    let service = common::create_test_app(pool, OpenSignups::Enabled, 1.0, 1.0).await;

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    let user = test::read_body_json::<RegisterResponse, _>(resp).await;

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
    let service = common::create_test_app(pool, OpenSignups::Enabled, 1.0, 1.0).await;

    // Register first user
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "duplicate_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Try to register second user with same username
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.89"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "duplicate_user",
            password: "different_password",
            password_verify: "different_password",
            email: "different_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Verify appropriate error response
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    // Check error message in response body
    let body = test::read_body(resp).await;
    let error: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error["error"], "username already exists");
}

#[sqlx::test(
    fixtures("with_test_user", "with_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_failures(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Disabled, 1.0, 1.0).await;

    // No key specified.  Should fail.
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // No invitation key provided when closed registration - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    // Invalid key specified.  Should fail.
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=invalid")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

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
    let service = common::create_test_app(pool, OpenSignups::Disabled, 1.0, 1.0).await;

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user2",
            password: "test_password2",
            password_verify: "test_password2",
            email: "newuser@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    let user = test::read_body_json::<RegisterResponse, _>(resp).await;

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
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user3",
            password: "test_password3",
            password_verify: "test_password3",
            email: "newuser2@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Invitation key already used - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures("with_test_user", "with_expired_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_expired_failure(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Disabled, 1.0, 1.0).await;

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user2",
            password: "test_password2",
            password_verify: "test_password2",
            email: "newuser@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Expired invitation key - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_authorized_endpoint_after_login(pool: PgPool) {
    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(token)
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

    let user = common::call_and_read_body_json::<MeResponse, _>(&service, req).await;

    assert_eq!(user.user.username, "test_user");
}
