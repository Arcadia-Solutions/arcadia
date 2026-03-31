pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::forum::{
    ForumCategory, ForumSubCategory, ReorderForumCategories, ReorderForumCategoryEntry,
    ReorderForumSubCategories, ReorderForumSubCategoryEntry, UserCreatedForumCategory,
    UserCreatedForumSubCategory,
};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;

// ============================================================================
// REORDER CATEGORY TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_reorder_categories(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::ForumCategoryFlow)
            .await;

    // Create three categories — they get incremental sort_order
    let mut category_ids = Vec::new();
    for name in ["Alpha", "Beta", "Gamma"] {
        let body = UserCreatedForumCategory { name: name.into() };
        let req = test::TestRequest::post()
            .uri("/api/forum/category")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .insert_header(auth_header(&staff.token))
            .set_json(&body)
            .to_request();
        let cat: ForumCategory =
            common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
        category_ids.push(cat.id);
    }

    // Reorder: Gamma first, Alpha second, Beta third
    let reorder_body = ReorderForumCategories {
        categories: vec![
            ReorderForumCategoryEntry {
                id: category_ids[2],
                sort_order: 1,
            },
            ReorderForumCategoryEntry {
                id: category_ids[0],
                sort_order: 2,
            },
            ReorderForumCategoryEntry {
                id: category_ids[1],
                sort_order: 3,
            },
        ],
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category/reorder")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&reorder_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify order via GET /api/forum
    let req = test::TestRequest::get()
        .uri("/api/forum")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .to_request();

    let overview: Value = common::call_and_read_body_json(&service, req).await;
    let categories = overview["forum_categories"].as_array().unwrap();

    let returned_names: Vec<&str> = categories
        .iter()
        .filter_map(|c| {
            let name = c["name"].as_str().unwrap();
            if ["Alpha", "Beta", "Gamma"].contains(&name) {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(returned_names, vec!["Gamma", "Alpha", "Beta"]);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_non_staff_cannot_reorder_categories(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let reorder_body = ReorderForumCategories {
        categories: vec![ReorderForumCategoryEntry {
            id: 100,
            sort_order: 5,
        }],
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/category/reorder")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&reorder_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ============================================================================
// REORDER SUB-CATEGORY TESTS
// ============================================================================

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_reorder_sub_categories(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        TestUser::ForumSubCategoryFlow,
    )
    .await;

    // Create three sub-categories in category 100
    let mut sub_category_ids = Vec::new();
    for name in ["Delta", "Epsilon", "Zeta"] {
        let body = UserCreatedForumSubCategory {
            forum_category_id: 100,
            name: name.into(),
            new_threads_restricted: false,
        };
        let req = test::TestRequest::post()
            .uri("/api/forum/sub-category")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .insert_header(auth_header(&staff.token))
            .set_json(&body)
            .to_request();
        let sub: ForumSubCategory =
            common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
        sub_category_ids.push(sub.id);
    }

    // Reorder: Zeta first, Epsilon second, Delta third
    let reorder_body = ReorderForumSubCategories {
        forum_category_id: 100,
        sub_categories: vec![
            ReorderForumSubCategoryEntry {
                id: sub_category_ids[2],
                sort_order: 1,
            },
            ReorderForumSubCategoryEntry {
                id: sub_category_ids[1],
                sort_order: 2,
            },
            ReorderForumSubCategoryEntry {
                id: sub_category_ids[0],
                sort_order: 3,
            },
        ],
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category/reorder")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&reorder_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify order via GET /api/forum
    let req = test::TestRequest::get()
        .uri("/api/forum")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .to_request();

    let overview: Value = common::call_and_read_body_json(&service, req).await;
    let categories = overview["forum_categories"].as_array().unwrap();

    let category = categories
        .iter()
        .find(|c| c["id"].as_i64().unwrap() == 100)
        .unwrap();
    let sub_categories = category["sub_categories"].as_array().unwrap();

    let returned_names: Vec<&str> = sub_categories
        .iter()
        .filter_map(|sc| {
            let name = sc["name"].as_str().unwrap();
            if ["Delta", "Epsilon", "Zeta"].contains(&name) {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(returned_names, vec!["Zeta", "Epsilon", "Delta"]);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_non_staff_cannot_reorder_sub_categories(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let reorder_body = ReorderForumSubCategories {
        forum_category_id: 100,
        sub_categories: vec![
            ReorderForumSubCategoryEntry {
                id: 101,
                sort_order: 1,
            },
            ReorderForumSubCategoryEntry {
                id: 100,
                sort_order: 2,
            },
        ],
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/sub-category/reorder")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&reorder_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ============================================================================
// SORT ORDER AUTO-ASSIGNMENT TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_created_categories_get_incremental_sort_order(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::ForumCategoryFlow)
            .await;

    let first = UserCreatedForumCategory {
        name: "First Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&first)
        .to_request();

    let first_category: ForumCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    let second = UserCreatedForumCategory {
        name: "Second Category".into(),
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&second)
        .to_request();

    let second_category: ForumCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(first_category.sort_order + 1, second_category.sort_order);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_forum_category"),
    migrations = "../storage/migrations"
)]
async fn test_created_sub_categories_get_incremental_sort_order(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        TestUser::ForumSubCategoryFlow,
    )
    .await;

    let first = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "First Sub Category".into(),
        new_threads_restricted: false,
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&first)
        .to_request();

    let first_sub_category: ForumSubCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    let second = UserCreatedForumSubCategory {
        forum_category_id: 100,
        name: "Second Sub Category".into(),
        new_threads_restricted: false,
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/sub-category")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&second)
        .to_request();

    let second_sub_category: ForumSubCategory =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(
        first_sub_category.sort_order + 1,
        second_sub_category.sort_order
    );
}
