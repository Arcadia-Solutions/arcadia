pub mod common;
pub mod mocks;

use crate::common::TestUser;
use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::user::UserLite;
use common::auth_header;
use common::create_test_app_and_login;
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_returns_results(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=alice")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    // Should find both alice_smith and alice_wonder
    assert_eq!(response.len(), 2);
    assert!(response.iter().any(|u| u.username == "alice_smith"));
    assert!(response.iter().any(|u| u.username == "alice_wonder"));
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_case_insensitive(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=ALICE")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    // Should find both alice_smith and alice_wonder regardless of case
    assert_eq!(response.len(), 2);
    assert!(response.iter().any(|u| u.username == "alice_smith"));
    assert!(response.iter().any(|u| u.username == "alice_wonder"));
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_partial_match(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=brown")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    // Should find charlie_brown
    assert_eq!(response.len(), 1);
    assert_eq!(response[0].username, "charlie_brown");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_returns_user_lite_fields(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=frank")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.len(), 1);
    let frank = &response[0];
    assert_eq!(frank.username, "frank_miller");
    assert!(frank.warned);
    assert!(!frank.banned);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_returns_banned_user(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=grace")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.len(), 1);
    let grace = &response[0];
    assert_eq!(grace.username, "grace_hopper");
    assert!(!grace.warned);
    assert!(grace.banned);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_no_results(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=nonexistent")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.len(), 0);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_users_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_users_lite_limits_to_five_results(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/users/lite?username=_")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: Vec<UserLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    // Should return at most 5 results even though there are more users with "_" in their username
    assert!(response.len() <= 5);
}
