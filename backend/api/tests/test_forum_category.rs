pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::forum::{
    EditedForumCategory, ForumCategory, UserCreatedForumCategory,
};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

// ============================================================================
// CREATE CATEGORY TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_staff_can_create_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::CreateForumCategory,
    )
    .await;

    let create_body = UserCreatedForumCategory {
        name: "New Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let category: ForumCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(category.name, "New Category");
    assert_eq!(category.created_by_id, 107);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_non_staff_cannot_create_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let create_body = UserCreatedForumCategory {
        name: "New Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_create_category_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let create_body = UserCreatedForumCategory {
        name: "New Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_create_category_with_empty_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::CreateForumCategory,
    )
    .await;

    let create_body = UserCreatedForumCategory { name: "".into() };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_create_category_with_whitespace_only_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::CreateForumCategory,
    )
    .await;

    let create_body = UserCreatedForumCategory { name: "   ".into() };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// EDIT CATEGORY TESTS
// ============================================================================

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumCategory,
    )
    .await;

    let edit_body = EditedForumCategory {
        id: 100,
        name: "Updated Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let category: ForumCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(category.id, 100);
    assert_eq!(category.name, "Updated Category Name");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_non_staff_cannot_edit_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumCategory {
        id: 100,
        name: "Updated Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_edit_category_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let edit_body = EditedForumCategory {
        id: 100,
        name: "Updated Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_edit_nonexistent_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumCategory,
    )
    .await;

    let edit_body = EditedForumCategory {
        id: 999,
        name: "Updated Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_edit_category_with_empty_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumCategory,
    )
    .await;

    let edit_body = EditedForumCategory {
        id: 100,
        name: "".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_edit_category_with_whitespace_only_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumCategory,
    )
    .await;

    let edit_body = EditedForumCategory {
        id: 100,
        name: "   ".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_create_and_edit_category_flow(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::ForumCategoryFlow,
    )
    .await;

    // Create a category
    let create_body = UserCreatedForumCategory {
        name: "Initial Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let category: ForumCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    let category_id = category.id;

    assert_eq!(category.name, "Initial Category");

    // Edit the category
    let edit_body = EditedForumCategory {
        id: category_id,
        name: "Edited Category".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let edited_category: ForumCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited_category.id, category_id);
    assert_eq!(edited_category.name, "Edited Category");
}
