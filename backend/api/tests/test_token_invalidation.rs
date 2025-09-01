pub mod common;
pub mod mocks;

use std::{sync::Arc, time::Duration};

use actix_web::{
    http::StatusCode,
    test::{call_service, read_body_json, TestRequest},
};
use arcadia_api::services::auth::InvalidationEntry;
use arcadia_storage::{connection_pool::ConnectionPool, redis::RedisInterface};
use serde_json::to_string;
use sqlx::PgPool;

use crate::{
    common::{create_test_app_and_login, Profile},
    mocks::mock_redis::{MockRedis, MockRedisPool},
};

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_reject_invalidated_tokens(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let redis_pool = MockRedisPool::default();
    let (service, user) = create_test_app_and_login(Arc::clone(&pool), redis_pool, 1.0, 1.0).await;

    // test that valid token by sending a request to an authenitcated endpoint
    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/me")
        .insert_header(("authorization", format!("Bearer {}", user.token.clone())))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let profile = read_body_json::<Profile, _>(resp).await;

    // invalidate user tokens
    let mut redis_conn = MockRedis::default();
    let entry = InvalidationEntry::new(profile.user.id);
    redis_conn
        .set(profile.user.id.to_string(), to_string(&entry).unwrap())
        .await
        .unwrap();

    // Add small delay so iat is at least 1 sec after the previous token's iat
    tokio::time::sleep(Duration::from_secs(1)).await;
    let (service, new_user) = create_test_app_and_login(
        Arc::clone(&pool),
        MockRedisPool::with_conn(redis_conn),
        1.0,
        1.0,
    )
    .await;

    // calls should with the old token are rejected
    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/me")
        .insert_header(("authorization", format!("Bearer {}", user.token.clone())))
        .to_request();
    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // but works with the new token
    let req = TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/users/me")
        .insert_header((
            "authorization",
            format!("Bearer {}", new_user.token.clone()),
        ))
        .to_request();

    let resp = call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
