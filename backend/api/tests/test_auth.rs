pub mod common;
pub mod mocks;

use actix_web::{
    http::{header::HeaderValue, StatusCode},
    test::{call_service, read_body, read_body_json, TestRequest},
};
use arcadia_api::services::auth::InvalidationEntry;
use arcadia_storage::{
    connection_pool::ConnectionPool, models::user::RefreshToken, redis::RedisInterface,
};
use mocks::mock_redis::MockRedisPool;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::PgPool;
use std::{sync::Arc, time::Duration};

use crate::common::TestUser;
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
    id: i32,
    username: String,
    email: String,
    registered_from_ip: String,
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_open_registration(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool, MockRedisPool::default()).await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "TestPassword123",
            password_verify: "TestPassword123",
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

    assert_eq!(user.username, "test_user");
    assert_eq!(user.email, "test_email@testdomain.com");
    // TODO: strip unnecessary /32 host postfix
    assert_eq!(user.registered_from_ip, "10.10.4.88/32");
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_duplicate_username_registration(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool, MockRedisPool::default()).await;

    // Register first user
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "duplicate_user",
            password: "TestPassword123",
            password_verify: "TestPassword123",
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
            password: "DifferentPassword456",
            password_verify: "DifferentPassword456",
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
    fixtures("with_test_users", "with_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_failures(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool, MockRedisPool::default()).await;

    // No key specified.  Should fail.
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "TestPassword123",
            password_verify: "TestPassword123",
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
            password: "TestPassword123",
            password_verify: "TestPassword123",
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
    fixtures("with_test_users", "with_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool, MockRedisPool::default()).await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user2",
            password: "TestPassword456",
            password_verify: "TestPassword456",
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

    assert_eq!(user.username, "test_user2");
    assert_eq!(user.email, "newuser@testdomain.com");
    // TODO: strip unnecessary /32 host postfix
    assert_eq!(user.registered_from_ip, "10.10.4.88/32");

    // Try again with same key.  Should fail.
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user3",
            password: "TestPassword789",
            password_verify: "TestPassword789",
            email: "newuser2@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;

    // Invitation key already used - returns BAD_REQUEST
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures("with_test_users", "with_expired_test_user_invite"),
    migrations = "../storage/migrations"
)]
async fn test_closed_registration_expired_failure(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool, MockRedisPool::default()).await;

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

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_authorized_endpoint_after_login(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

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

    assert_eq!(user.user.username, "user_basic");
}

#[sqlx::test(
    fixtures("with_test_banned_user"),
    migrations = "../storage/migrations"
)]
async fn test_login_with_banned_user(pool: PgPool) {
    let service = create_test_app(
        Arc::new(ConnectionPool::with_pg_pool(pool)),
        MockRedisPool::default(),
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

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_refresh_with_invalidated_token(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        Arc::clone(&pool),
        MockRedisPool::default(),
        TestUser::Standard,
    )
    .await;

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
        TestUser::Standard,
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

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_banned_user_token_invalidation(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Create shared Redis pool and mock connection
    let redis_conn = MockRedis::default();
    let redis_pool = MockRedisPool::with_conn(redis_conn);

    // Login with regular user
    let service = create_test_app(Arc::clone(&pool), redis_pool).await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_basic",
            "password": "test_password",
            "remember_me": true,
        }))
        .to_request();

    let regular_user =
        call_and_read_body_json::<arcadia_storage::models::user::LoginResponse, _>(&service, req)
            .await;

    // Verify regular user can access authenticated endpoints
    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/me")
        .insert_header(auth_header(&regular_user.token))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let profile = read_body_json::<Profile, _>(resp).await;
    let regular_user_id = profile.user.id;

    // Login with admin user who has ban permissions
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_warn_ban",
            "password": "test_password",
            "remember_me": true,
        }))
        .to_request();

    let admin_user =
        call_and_read_body_json::<arcadia_storage::models::user::LoginResponse, _>(&service, req)
            .await;

    // Ban the regular user via the warn endpoint
    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/warn")
        .insert_header(auth_header(&admin_user.token))
        .set_json(serde_json::json!({
            "user_id": regular_user_id,
            "reason": "Test ban",
            "ban": true,
        }))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Try to make a request with the banned user's token - should fail
    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/me")
        .insert_header(auth_header(&regular_user.token))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(
    fixtures("with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_registration_assigns_class_permissions(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Set the default class to test_class which has upload_torrent and download_torrent permissions
    let mut settings = pool.get_arcadia_settings().await.unwrap();
    settings.user_class_name_on_signup = "test_class".to_string();
    pool.update_arcadia_settings(&settings).await.unwrap();

    let service = create_test_app(pool.clone(), MockRedisPool::default()).await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "test_user_perms",
            password: "TestPassword123",
            password_verify: "TestPassword123",
            email: "test_perms@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Get the user ID from the registration response
    let registered_user = read_body_json::<RegisterResponse, _>(resp).await;

    // Verify the user was created with the class permissions
    let user = pool.find_user_with_id(registered_user.id).await.unwrap();

    assert_eq!(user.permissions.len(), 2);
    assert!(user
        .permissions
        .contains(&arcadia_storage::models::user::UserPermission::UploadTorrent));
    assert!(user
        .permissions
        .contains(&arcadia_storage::models::user::UserPermission::DownloadTorrent));
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_registration_sends_automated_message_when_configured(pool: PgPool) {
    let sender_id = 1; // user from fixture
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Configure automated signup message
    let mut settings = pool.get_arcadia_settings().await.unwrap();
    settings.automated_message_on_signup = Some("Welcome to the site!".to_string());
    settings.automated_message_on_signup_sender_id = Some(sender_id);
    settings.automated_message_on_signup_locked = Some(true);
    settings.automated_message_on_signup_conversation_name = Some("Welcome".to_string());
    pool.update_arcadia_settings(&settings).await.unwrap();

    let service = create_test_app(pool.clone(), MockRedisPool::default()).await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "new_user_msg",
            password: "TestPassword123",
            password_verify: "TestPassword123",
            email: "newuser_msg@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let user = read_body_json::<RegisterResponse, _>(resp).await;
    assert_eq!(user.username, "new_user_msg");

    // Verify conversation was created using repository function
    let conversations = pool.find_user_conversations(user.id).await.unwrap();
    let conversations_array = conversations.as_array().unwrap();

    assert_eq!(conversations_array.len(), 1);
    let conversation = &conversations_array[0];

    assert_eq!(conversation["subject"].as_str().unwrap(), "Welcome");
    assert!(conversation["locked"].as_bool().unwrap());
    assert_eq!(
        conversation["sender_id"].as_i64().unwrap(),
        sender_id as i64
    );

    // Verify message content using find_conversation
    let conversation_id = conversation["id"].as_i64().unwrap();
    let conversation_details = pool
        .find_conversation(conversation_id, user.id, false)
        .await
        .unwrap();

    let messages = conversation_details["messages"].as_array().unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(
        messages[0]["content"].as_str().unwrap(),
        "Welcome to the site!"
    );
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_registration_no_automated_message_when_not_configured(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool.clone(), MockRedisPool::default()).await;

    let req = TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/register")
        .set_json(RegisterRequest {
            username: "new_user_no_msg",
            password: "TestPassword123",
            password_verify: "TestPassword123",
            email: "newuser_nomsg@testdomain.com",
        })
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Get the user ID from the registration response
    let user = read_body_json::<RegisterResponse, _>(resp).await;

    // Verify no conversation was created for the new user
    let conversations = pool.find_user_conversations(user.id).await.unwrap();
    let conversations_array = conversations.as_array().unwrap();

    assert_eq!(conversations_array.len(), 0);
}
