pub mod common;
pub mod mocks;

use std::sync::Arc;

use actix_web::{http::StatusCode, test};
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::{common::PaginatedResults, torrent_request::TorrentRequestWithTitleGroupLite},
};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;

use crate::common::{auth_header, TestUser};

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_torrent_request"
    ),
    migrations = "../storage/migrations"
)]
async fn test_search_torrent_requests(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/search/torrent-requests?title_group_name=Love")
        .to_request();

    let resp = test::call_service(&service, req).await;

    if resp.status() != StatusCode::OK {
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);
        eprintln!("Error response: {}", body_str);
        panic!("Expected 200 OK, got 500");
    }

    let results: PaginatedResults<TorrentRequestWithTitleGroupLite> =
        test::read_body_json(resp).await;

    assert_eq!(results.results.len(), 1);
    assert_eq!(results.results[0].torrent_request.id, 1);
    assert_eq!(
        results.results[0].title_group.name,
        "Love Me Do / P.S. I Love You"
    );
}
