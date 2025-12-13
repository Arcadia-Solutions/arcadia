pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::common::PaginatedResults;
use arcadia_storage::models::forum::{
    EditedForumThread, ForumPost, ForumPostHierarchy, ForumSubCategoryHierarchy, ForumThread,
    ForumThreadEnriched, UserCreatedForumPost, UserCreatedForumThread,
};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

// ============================================================================
// CREATE THREAD TESTS
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_create_thread_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "My First Thread".into(),
        first_post: UserCreatedForumPost {
            content: "This is the first post content".into(),
            forum_thread_id: 0, // Will be set by the server
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let thread: ForumThread =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(thread.name, "My First Thread");
    assert_eq!(thread.forum_sub_category_id, 100);
    assert_eq!(thread.created_by_id, 100);
    assert_eq!(thread.posts_amount, 1);
    assert!(!thread.sticky);
    assert!(!thread.locked);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_create_thread_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "Should Fail".into(),
        first_post: UserCreatedForumPost {
            content: "This should not be created".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_create_thread_with_empty_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "".into(),
        first_post: UserCreatedForumPost {
            content: "First post".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_create_thread_with_empty_post(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "Thread Name".into(),
        first_post: UserCreatedForumPost {
            content: "".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_create_thread_with_invalid_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 999, // Non-existent sub-category
        name: "Thread Name".into(),
        first_post: UserCreatedForumPost {
            content: "First post".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    // Should fail due to foreign key constraint
    assert!(
        resp.status() == StatusCode::INTERNAL_SERVER_ERROR
            || resp.status() == StatusCode::BAD_REQUEST
    );
}

// ============================================================================
// GET THREAD TESTS
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_thread_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread?id=100")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let thread: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(thread.id, 100);
    assert_eq!(thread.name, "Test Thread");
    assert_eq!(thread.forum_sub_category_id, 100);
    assert_eq!(thread.forum_sub_category_name, "Test Sub Category");
    assert_eq!(thread.forum_category_name, "Test Category");
    assert_eq!(thread.created_by_id, 100);
    assert!(!thread.sticky);
    assert!(!thread.locked);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_thread_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread?id=100")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_get_nonexistent_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread?id=999")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// EDIT THREAD TESTS - OWNERSHIP
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_owner_can_edit_thread_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumThread {
        id: 100,
        forum_sub_category_id: 100,
        name: "Updated Thread Name".into(),
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.id, 100);
    assert_eq!(edited.name, "Updated Thread Name");
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_owner_can_toggle_sticky(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumThread {
        id: 100,
        forum_sub_category_id: 100,
        name: "Test Thread".into(),
        sticky: true, // Set sticky to true
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert!(edited.sticky);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_owner_can_toggle_locked(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumThread {
        id: 100,
        forum_sub_category_id: 100,
        name: "Test Thread".into(),
        sticky: false,
        locked: true, // Set locked to true
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert!(edited.locked);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_owner_can_move_thread_to_different_sub_category(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumThread {
        id: 100,
        forum_sub_category_id: 101, // Move to different sub-category
        name: "Test Thread".into(),
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.forum_sub_category_id, 101);
    assert_eq!(edited.forum_sub_category_name, "Test Sub Category 2");
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_user2",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_non_owner_cannot_edit_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // First, login as staff user (101) and create a thread owned by them
    let (service, staff_user) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), 100, 100, TestUser::Staff).await;

    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "Staff User Thread".into(),
        first_post: UserCreatedForumPost {
            content: "Thread created by staff user".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff_user.token))
        .set_json(&create_body)
        .to_request();

    let thread: ForumThread =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Now login as a different non-staff user (100) and try to edit the staff user's thread
    let (service2, standard_user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard).await;

    let edit_body = EditedForumThread {
        id: thread.id, // Thread owned by user 101 (staff)
        forum_sub_category_id: 100,
        name: "Unauthorized Edit Attempt".into(),
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&standard_user.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service2, req).await;
    // Non-owner, non-staff user should not be able to edit the thread
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_user2",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_any_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Thread 100 is owned by user 100 (standard user)
    // Login as staff user
    let (service, staff) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Staff).await;

    let edit_body = EditedForumThread {
        id: 100, // Thread owned by user 100
        forum_sub_category_id: 100,
        name: "Staff Edited Thread".into(),
        sticky: true,
        locked: true,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.name, "Staff Edited Thread");
    assert!(edited.sticky);
    assert!(edited.locked);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_thread_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let edit_body = EditedForumThread {
        id: 100,
        forum_sub_category_id: 100,
        name: "Should Fail".into(),
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_edit_nonexistent_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumThread {
        id: 999, // Non-existent thread
        forum_sub_category_id: 100,
        name: "Should Fail".into(),
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_thread_with_empty_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let edit_body = EditedForumThread {
        id: 100,
        forum_sub_category_id: 100,
        name: "".into(), // Empty name
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// THREAD LOCKING TESTS
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_cannot_post_in_locked_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    // Thread 101 is locked
    let post_body = UserCreatedForumPost {
        content: "This should fail".into(),
        forum_thread_id: 101,
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&post_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_can_unlock_and_post(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    // First unlock the locked thread (101)
    let edit_body = EditedForumThread {
        id: 101,
        forum_sub_category_id: 100,
        name: "Locked Thread".into(),
        sticky: false,
        locked: false, // Unlock it
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;
    assert!(!edited.locked);

    // Now try to post in it
    let post_body = UserCreatedForumPost {
        content: "This should succeed now".into(),
        forum_thread_id: 101,
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&post_body)
        .to_request();

    let post: ForumPost =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    assert_eq!(post.content, "This should succeed now");
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_create_thread_add_posts_edit_thread_flow(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    // Create thread
    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "Integration Test Thread".into(),
        first_post: UserCreatedForumPost {
            content: "First post content".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let thread: ForumThread =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    let thread_id = thread.id;

    // Add a second post
    let post_body = UserCreatedForumPost {
        content: "Second post content".into(),
        forum_thread_id: thread_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&post_body)
        .to_request();

    let _post: ForumPost =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Edit the thread
    let edit_body = EditedForumThread {
        id: thread_id,
        forum_sub_category_id: 100,
        name: "Updated Integration Test Thread".into(),
        sticky: true,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.name, "Updated Integration Test Thread");
    assert!(edited.sticky);
    assert_eq!(edited.posts_amount, 2);

    // Get the thread to verify all changes
    let req = test::TestRequest::get()
        .uri(&format!("/api/forum/thread?id={}", thread_id))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let fetched: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(fetched.name, "Updated Integration Test Thread");
    assert!(fetched.sticky);
    assert_eq!(fetched.posts_amount, 2);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_move_thread_with_posts_between_sub_categories(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    // Create thread in sub-category 100
    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "Thread to Move".into(),
        first_post: UserCreatedForumPost {
            content: "First post".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let thread: ForumThread =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    assert_eq!(thread.forum_sub_category_id, 100);

    // Add a post
    let post_body = UserCreatedForumPost {
        content: "Second post".into(),
        forum_thread_id: thread.id,
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&post_body)
        .to_request();

    let _: ForumPost =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Move thread to sub-category 101
    let edit_body = EditedForumThread {
        id: thread.id,
        forum_sub_category_id: 101,
        name: "Thread to Move".into(),
        sticky: false,
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: ForumThreadEnriched =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.forum_sub_category_id, 101);
    assert_eq!(edited.forum_sub_category_name, "Test Sub Category 2");
    assert_eq!(edited.posts_amount, 2); // Posts should remain intact
}

// ============================================================================
// GET THREAD POSTS TESTS (PAGINATION)
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_thread_posts_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=100&page_size=10")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let posts: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert!(posts.total_items > 0);
    assert!(!posts.results.is_empty());
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_thread_posts_with_multiple_posts_pagination(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    // Create a thread
    let create_body = UserCreatedForumThread {
        forum_sub_category_id: 100,
        name: "Thread with Many Posts".into(),
        first_post: UserCreatedForumPost {
            content: "First post".into(),
            forum_thread_id: 0,
        },
    };

    let req = test::TestRequest::post()
        .uri("/api/forum/thread")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let thread: ForumThread =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Add 5 more posts
    for i in 2..=6 {
        let post_body = UserCreatedForumPost {
            content: format!("Post number {}", i),
            forum_thread_id: thread.id,
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/post")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .insert_header(auth_header(&user.token))
            .set_json(&post_body)
            .to_request();

        let _: ForumPost =
            common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    }

    // Get first page with page_size=3
    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/forum/thread/posts?thread_id={}&page=1&page_size=3",
            thread.id
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let page1: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(page1.results.len(), 3);
    assert_eq!(page1.total_items, 6);
    assert_eq!(page1.page, 1);

    // Get second page
    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/forum/thread/posts?thread_id={}&page=2&page_size=3",
            thread.id
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let page2: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(page2.results.len(), 3);
    assert_eq!(page2.page, 2);

    // Ensure different posts on different pages
    assert_ne!(page1.results[0].id, page2.results[0].id);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_thread_posts_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=100&page_size=10")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_get_posts_for_nonexistent_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=999&page_size=10")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let resp = test::call_service(&service, req).await;
    // Should return empty results or error
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
}

// ============================================================================
// GET SUB-CATEGORY THREADS TESTS
// ============================================================================

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_sub_category_threads_success(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/sub-category?id=100")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let sub_category: ForumSubCategoryHierarchy =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(sub_category.id, 100);
    assert_eq!(sub_category.name, "Test Sub Category");
    assert!(sub_category.threads.is_some());

    let threads = sub_category.threads.unwrap();
    // Should have at least the test threads (100, 101, 102 are in sub-category 100)
    assert!(threads.len() >= 3);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_sub_category_threads_show_sticky_threads(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/sub-category?id=100")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let sub_category: ForumSubCategoryHierarchy =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let threads = sub_category.threads.unwrap();

    // Thread 102 is sticky
    let sticky_thread = threads.iter().find(|t| t.id == 102);
    assert!(sticky_thread.is_some());
    assert!(sticky_thread.unwrap().sticky);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_sub_category_threads_show_locked_threads(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/sub-category?id=100")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let sub_category: ForumSubCategoryHierarchy =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let threads = sub_category.threads.unwrap();

    // Thread 101 is locked
    let locked_thread = threads.iter().find(|t| t.id == 101);
    assert!(locked_thread.is_some());
    assert!(locked_thread.unwrap().locked);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_sub_category_threads_without_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        arcadia_api::OpenSignups::Disabled,
        100,
        100,
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/sub-category?id=100")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_get_nonexistent_sub_category_threads(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Standard)
            .await;

    let req = test::TestRequest::get()
        .uri("/api/forum/sub-category?id=999")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
