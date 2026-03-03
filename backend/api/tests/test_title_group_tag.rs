pub mod common;
pub mod mocks;

use crate::common::TestUser;
use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::common::PaginatedResults;
use arcadia_storage::models::title_group_tag::{
    DeleteTitleGroupTagRequest, TitleGroupTag, TitleGroupTagLite,
};
use common::{auth_header, create_test_app_and_login};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group_tag"),
    migrations = "../storage/migrations"
)]
async fn test_soft_delete_tag_then_recreate_is_rejected(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Step 1: delete the tag (soft delete)
    let (service, delete_user) = create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::DeleteTitleGroupTag,
    )
    .await;

    let delete_body = DeleteTitleGroupTagRequest {
        id: 1,
        deletion_reason: "duplicate of adventure".into(),
    };

    let req = test::TestRequest::delete()
        .uri("/api/title-group-tags")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&delete_user.token))
        .set_json(&delete_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Step 2: verify the tag no longer appears in search results
    let req = test::TestRequest::get()
        .uri("/api/search/title-group-tags/lite?name=action&page=1&page_size=10")
        .insert_header(auth_header(&delete_user.token))
        .to_request();

    let response: PaginatedResults<TitleGroupTagLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.results.len(), 0);

    // Step 3: attempt to recreate the same tag — should fail with the deletion reason
    let (service, standard_user) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::post()
        .uri("/api/title-group-tags")
        .insert_header(auth_header(&standard_user.token))
        .set_json(serde_json::json!({"name": "action"}))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group_tag"),
    migrations = "../storage/migrations"
)]
async fn test_create_tag_returns_existing_non_deleted_tag(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::post()
        .uri("/api/title-group-tags")
        .insert_header(auth_header(&user.token))
        .set_json(serde_json::json!({"name": "action"}))
        .to_request();

    let response: TitleGroupTag =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(response.name, "action");
    assert_eq!(response.id, 1);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_title_group_tag",
        "with_test_title_group_tag_applied"
    ),
    migrations = "../storage/migrations"
)]
async fn test_delete_tag_removes_it_from_all_title_groups(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Verify the tag is initially applied to the title group
    let title_group = pool.find_title_group(1).await.unwrap();
    assert!(title_group.tags.contains(&"action".to_string()));

    // Delete the tag
    let (service, delete_user) = create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::DeleteTitleGroupTag,
    )
    .await;

    let delete_body = DeleteTitleGroupTagRequest {
        id: 1,
        deletion_reason: "no longer needed".into(),
    };

    let req = test::TestRequest::delete()
        .uri("/api/title-group-tags")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&delete_user.token))
        .set_json(&delete_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify the tag has been removed from the title group
    let title_group = pool.find_title_group(1).await.unwrap();
    assert!(title_group.tags.is_empty());
}
