//! Access control of the tracker endpoints (`/api/tracker`): the api key, and the single
//! address the instance may restrict them to.

pub mod common;
pub mod mocks;

use crate::common::build_test_app;
use crate::mocks::mock_redis::MockRedisPool;
use actix_web::http::StatusCode;
use actix_web::test::{call_service, TestRequest};
use arcadia_api::config::Config;
use arcadia_storage::connection_pool::ConnectionPool;
use sqlx::PgPool;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

/// The tracker api key of the configuration the test application loads.
const TRACKER_API_KEY: &str = "change_me";

const ALLOWED_ADDRESS: &str = "10.10.4.88";
const OTHER_ADDRESS: &str = "10.10.4.89";

fn allow_only(address: Option<Ipv4Addr>) -> impl FnOnce(&mut Config) {
    move |config| {
        // The test requests carry no peer address, so the client address can only come from a
        // header.
        config.api.reverse_proxy_client_ip_header_name = Some("X-Forwarded-For".to_owned());
        config.tracker.allowed_ip = address.map(IpAddr::V4);
    }
}

fn tracker_request(address: &str) -> actix_http::Request {
    TestRequest::get()
        .insert_header(("X-Forwarded-For", address))
        .insert_header(("api_key", TRACKER_API_KEY))
        .uri("/api/tracker/announce")
        .to_request()
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_the_allowed_address_reaches_the_tracker_endpoints(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = build_test_app(
        pool,
        MockRedisPool::default(),
        allow_only(Some(Ipv4Addr::new(10, 10, 4, 88))),
    )
    .await;

    // No tracker route is registered yet, so authenticating successfully lands on the router,
    // which answers 404. What matters is that authentication let the request through.
    assert_eq!(
        call_service(&service, tracker_request(ALLOWED_ADDRESS))
            .await
            .status(),
        StatusCode::NOT_FOUND
    );
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_another_address_is_refused_even_with_the_right_api_key(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = build_test_app(
        pool,
        MockRedisPool::default(),
        allow_only(Some(Ipv4Addr::new(10, 10, 4, 88))),
    )
    .await;

    assert_eq!(
        call_service(&service, tracker_request(OTHER_ADDRESS))
            .await
            .status(),
        StatusCode::UNAUTHORIZED,
        "the api key alone must not be enough once an address is configured"
    );
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_an_unknown_address_is_refused(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = build_test_app(
        pool,
        MockRedisPool::default(),
        allow_only(Some(Ipv4Addr::new(10, 10, 4, 88))),
    )
    .await;

    // Neither a usable header nor a peer address: the allow list has to fail closed.
    let request = TestRequest::get()
        .insert_header(("api_key", TRACKER_API_KEY))
        .uri("/api/tracker/announce")
        .to_request();

    assert_eq!(
        call_service(&service, request).await.status(),
        StatusCode::UNAUTHORIZED
    );
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_every_address_is_accepted_when_none_is_configured(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = build_test_app(pool, MockRedisPool::default(), allow_only(None)).await;

    for address in [ALLOWED_ADDRESS, OTHER_ADDRESS] {
        assert_eq!(
            call_service(&service, tracker_request(address))
                .await
                .status(),
            StatusCode::NOT_FOUND,
            "{address} should reach the tracker endpoints when no address is configured"
        );
    }
}

#[sqlx::test(migrations = "../storage/migrations")]
async fn test_a_wrong_api_key_is_refused_from_the_allowed_address(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = build_test_app(
        pool,
        MockRedisPool::default(),
        allow_only(Some(Ipv4Addr::new(10, 10, 4, 88))),
    )
    .await;

    let request = TestRequest::get()
        .insert_header(("X-Forwarded-For", ALLOWED_ADDRESS))
        .insert_header(("api_key", "not_the_tracker_api_key"))
        .uri("/api/tracker/announce")
        .to_request();

    assert_eq!(
        call_service(&service, request).await.status(),
        StatusCode::UNAUTHORIZED
    );
}
