pub mod common;
pub mod mocks;

use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::bonus_points_log::{BonusPointsLog, BonusPointsLogAction};
use arcadia_storage::models::common::PaginatedResults;
use arcadia_storage::models::shop::BuyUploadRequest;
use common::{auth_header, call_and_read_body_json, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

const BYTES_PER_GB: i64 = 1_073_741_824;

fn search_uri(actions: &str) -> String {
    format!(
        "/api/users/bonus-points-logs?page=1&page_size=50\
         &order_by_column=created_at&order_by_direction=desc\
         &from_date=2000-01-01T00:00:00Z&to_date=2100-01-01T00:00:00Z\
         &actions={actions}"
    )
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_shop_balance"),
    migrations = "../storage/migrations"
)]
async fn test_search_bonus_points_logs_returns_shop_purchase(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        Arc::clone(&pool),
        MockRedisPool::default(),
        TestUser::Standard,
    )
    .await;

    let buy = test::TestRequest::post()
        .uri("/api/shop/buy-upload")
        .insert_header(auth_header(&user.token))
        .set_json(&BuyUploadRequest {
            bytes: 5 * BYTES_PER_GB,
        })
        .to_request();
    let resp = test::call_service(&service, buy).await;
    assert!(resp.status().is_success());

    let all = test::TestRequest::get()
        .uri(&search_uri(""))
        .insert_header(auth_header(&user.token))
        .to_request();
    let results: PaginatedResults<BonusPointsLog> = call_and_read_body_json(&service, all).await;

    assert_eq!(results.total_items, 1);
    assert_eq!(results.results.len(), 1);
    let entry = &results.results[0];
    assert_eq!(entry.user_id, 100);
    assert_eq!(entry.amount, -500);
    assert!(matches!(
        entry.action,
        BonusPointsLogAction::ShopPurchaseUpload
    ));

    let filtered = test::TestRequest::get()
        .uri(&search_uri("gift_received"))
        .insert_header(auth_header(&user.token))
        .to_request();
    let filtered: PaginatedResults<BonusPointsLog> =
        call_and_read_body_json(&service, filtered).await;
    assert_eq!(filtered.total_items, 0);
    assert!(filtered.results.is_empty());
}
