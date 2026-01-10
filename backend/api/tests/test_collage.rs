pub mod common;
pub mod mocks;

use crate::common::TestUser;
use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::collage::{Collage, EditedCollage};
use common::{auth_header, create_test_app_and_login};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_collage"),
    migrations = "../storage/migrations"
)]
async fn test_owner_can_edit_own_collage(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::put()
        .uri("/api/collages")
        .insert_header(auth_header(&user.token))
        .set_json(EditedCollage {
            id: 1,
            name: "Updated Name".into(),
            cover: None,
            description: "Updated".into(),
            tags: vec!["updated".into()],
            category: arcadia_storage::models::collage::CollageCategory::Personal,
        })
        .to_request();

    let resp: Collage =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;
    assert_eq!(resp.name, "Updated Name");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_collage"),
    migrations = "../storage/migrations"
)]
async fn test_user_cannot_edit_others_collage(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::put()
        .uri("/api/collages")
        .insert_header(auth_header(&user.token))
        .set_json(EditedCollage {
            id: 2, // owned by user 101
            name: "Hacked".into(),
            cover: None,
            description: "Hacked".into(),
            tags: vec![],
            category: arcadia_storage::models::collage::CollageCategory::Personal,
        })
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_collage"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_any_collage(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::EditCollage).await;

    let req = test::TestRequest::put()
        .uri("/api/collages")
        .insert_header(auth_header(&user.token))
        .set_json(EditedCollage {
            id: 1,
            name: "Staff Edit".into(),
            cover: None,
            description: "Edited by staff".into(),
            tags: vec![],
            category: arcadia_storage::models::collage::CollageCategory::Theme,
        })
        .to_request();

    let resp: Collage =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;
    assert_eq!(resp.name, "Staff Edit");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_collage"),
    migrations = "../storage/migrations"
)]
async fn test_user_cannot_delete_collage(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::delete()
        .uri("/api/collages?collage_id=1")
        .insert_header(auth_header(&user.token))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_collage"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_delete_collage(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::DeleteCollage,
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/collages?collage_id=1")
        .insert_header(auth_header(&user.token))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify deletion
    assert!(pool.find_collage(&1).await.is_err());
}
