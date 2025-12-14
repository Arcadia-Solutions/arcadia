pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::forum::{
    EditedForumSubCategory, ForumSubCategory, UserCreatedForumSubCategory,
};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

// ============================================================================
// CREATE SUB-CATEGORY TESTS
// ============================================================================

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_create_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::CreateForumSubCategory,
    )
    .await;

    let create_body = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "New Sub Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let sub_category: ForumSubCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(sub_category.name, "New Sub Category");
    assert_eq!(sub_category.forum_category_id, 100);
    assert_eq!(sub_category.created_by_id, 109);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_non_staff_cannot_create_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let create_body = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "New Sub Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_create_sub_category_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let create_body = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "New Sub Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_create_sub_category_with_empty_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::CreateForumSubCategory,
    )
    .await;

    let create_body = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_create_sub_category_with_whitespace_only_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::CreateForumSubCategory,
    )
    .await;

    let create_body = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "   ".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// EDIT SUB-CATEGORY TESTS
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumSubCategory,
    )
    .await;

    let edit_body = EditedForumSubCategory {
        id: 100,
        name: "Updated Sub Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let sub_category: ForumSubCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(sub_category.id, 100);
    assert_eq!(sub_category.name, "Updated Sub Category Name");
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_non_staff_cannot_edit_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumSubCategory {
        id: 100,
        name: "Updated Sub Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_sub_category_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let edit_body = EditedForumSubCategory {
        id: 100,
        name: "Updated Sub Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_edit_nonexistent_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumSubCategory,
    )
    .await;

    let edit_body = EditedForumSubCategory {
        id: 999,
        name: "Updated Sub Category Name".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_sub_category_with_empty_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumSubCategory,
    )
    .await;

    let edit_body = EditedForumSubCategory {
        id: 100,
        name: "".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_sub_category_with_whitespace_only_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::EditForumSubCategory,
    )
    .await;

    let edit_body = EditedForumSubCategory {
        id: 100,
        name: "   ".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
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

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_create_and_edit_sub_category_flow(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        101,
        101,
        TestUser::ForumSubCategoryFlow,
    )
    .await;

    // Create a sub category
    let create_body = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "Initial Sub Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&create_body)
        .to_request();

    let sub_category: ForumSubCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    let sub_category_id = sub_category.id;

    assert_eq!(sub_category.name, "Initial Sub Category");
    assert_eq!(sub_category.forum_category_id, 100);

    // Edit the sub category
    let edit_body = EditedForumSubCategory {
        id: sub_category_id,
        name: "Edited Sub Category".into(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let edited_sub_category: ForumSubCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited_sub_category.id, sub_category_id);
    assert_eq!(edited_sub_category.name, "Edited Sub Category");
}
