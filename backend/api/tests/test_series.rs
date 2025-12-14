pub mod common;
pub mod mocks;

use std::sync::Arc;

use actix_web::{
    http::StatusCode,
    test::{self, call_service},
};
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::{
        series::{EditedSeries, Series},
        title_group::TitleGroupAndAssociatedData,
    },
};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;

use crate::common::{
    auth_header, call_and_read_body_json_with_status, create_test_app_and_login, TestUser,
};

#[sqlx::test(
    fixtures("with_test_users", "with_test_series"),
    migrations = "../storage/migrations"
)]
async fn test_edit_series(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        100,
        100,
        TestUser::EditSeries,
    )
    .await;

    let payload = EditedSeries {
        id: 1,
        name: "Updated Series".to_string(),
        description: "Updated description".to_string(),
        covers: vec!["https://example.com/updated-cover.jpg".to_string()],
        banners: vec!["https://example.com/updated-banner.jpg".to_string()],
        tags: vec!["updated".to_string()],
    };

    let req = test::TestRequest::put()
        .uri("/api/series")
        .insert_header(auth_header(&user.token))
        .set_json(&payload)
        .to_request();

    let series: Series = call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(series.name, payload.name);
    assert_eq!(series.description, payload.description);
    assert_eq!(series.covers, payload.covers);
    assert_eq!(series.banners, Some(payload.banners));
    assert_eq!(series.tags, payload.tags);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_series", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_add_title_group_to_series(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::post()
        .uri("/api/series/title-group")
        .insert_header(auth_header(&user.token))
        .set_json(serde_json::json!({
            "series_id": 1,
            "title_group_id": 1
        }))
        .to_request();

    let _ = call_service(&service, req).await;

    let req = test::TestRequest::get()
        .uri("/api/title-groups?id=1")
        .insert_header(auth_header(&user.token))
        .to_request();

    let title_group: TitleGroupAndAssociatedData =
        call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(title_group.title_group.id, 1);
    assert_eq!(title_group.title_group.series_id, Some(1));
}
