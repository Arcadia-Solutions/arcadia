pub mod common;
pub mod mocks;

use actix_web::{http::StatusCode, test};
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::user::{
        DeleteUserClass, EditedUserClass, UpdatedUserPermissions, UserClass, UserClassChange,
        UserClassLockStatus, UserCreatedUserClass, UserPermission,
    },
};
use common::{
    auth_header, call_and_read_body_json_with_status, create_test_app, create_test_app_and_login,
    TestUser,
};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

// ============================================================================
// CREATE USER CLASS TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_staff_can_create_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::CreateUserClass).await;

    let user_class = UserCreatedUserClass {
        name: "power_user".into(),
        default_permissions: vec![
            UserPermission::UploadTorrent,
            UserPermission::DownloadTorrent,
        ],
        automatic_promotion: true,
        automatic_demotion: true,
        promotion_allowed_while_warned: false,
        required_account_age_in_days: 0,
        required_ratio: 0.0,
        required_torrent_uploads: 0,
        required_torrent_uploads_in_unique_title_groups: 0,
        required_uploaded: 0,
        required_torrent_snatched: 0,
        required_downloaded: 0,
        required_forum_posts: 0,
        required_forum_posts_in_unique_threads: 0,
        required_title_group_comments: 0,
        required_seeding_size: 0,
    };

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes")
        .set_json(&user_class)
        .to_request();

    let created: UserClass =
        call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    assert_eq!(created.name, "power_user");
    assert_eq!(created.default_permissions.len(), 2);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_regular_user_cannot_create_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let user_class = UserCreatedUserClass {
        name: "power_user".into(),
        default_permissions: vec![],
        automatic_promotion: true,
        automatic_demotion: true,
        promotion_allowed_while_warned: false,
        required_account_age_in_days: 0,
        required_ratio: 0.0,
        required_torrent_uploads: 0,
        required_torrent_uploads_in_unique_title_groups: 0,
        required_uploaded: 0,
        required_torrent_snatched: 0,
        required_downloaded: 0,
        required_forum_posts: 0,
        required_forum_posts_in_unique_threads: 0,
        required_title_group_comments: 0,
        required_seeding_size: 0,
    };

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes")
        .set_json(&user_class)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_create_user_class_requires_auth(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = create_test_app(pool, MockRedisPool::default()).await;

    let user_class = UserCreatedUserClass {
        name: "power_user".into(),
        default_permissions: vec![],
        automatic_promotion: true,
        automatic_demotion: true,
        promotion_allowed_while_warned: false,
        required_account_age_in_days: 0,
        required_ratio: 0.0,
        required_torrent_uploads: 0,
        required_torrent_uploads_in_unique_title_groups: 0,
        required_uploaded: 0,
        required_torrent_snatched: 0,
        required_downloaded: 0,
        required_forum_posts: 0,
        required_forum_posts_in_unique_threads: 0,
        required_title_group_comments: 0,
        required_seeding_size: 0,
    };

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/user-classes")
        .set_json(&user_class)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_create_user_class_with_invalid_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::CreateUserClass).await;

    // Too short name
    let user_class = UserCreatedUserClass {
        name: "ab".into(),
        default_permissions: vec![],
        automatic_promotion: true,
        automatic_demotion: true,
        promotion_allowed_while_warned: false,
        required_account_age_in_days: 0,
        required_ratio: 0.0,
        required_torrent_uploads: 0,
        required_torrent_uploads_in_unique_title_groups: 0,
        required_uploaded: 0,
        required_torrent_snatched: 0,
        required_downloaded: 0,
        required_forum_posts: 0,
        required_forum_posts_in_unique_threads: 0,
        required_title_group_comments: 0,
        required_seeding_size: 0,
    };

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes")
        .set_json(&user_class)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// EDIT USER CLASS TESTS
// ============================================================================

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::EditUserClass).await;

    let edited = EditedUserClass {
        name: "advanced_user".into(),
        default_permissions: vec![UserPermission::EditArtist],
        automatic_promotion: true,
        automatic_demotion: true,
        promotion_allowed_while_warned: false,
        required_account_age_in_days: 0,
        required_ratio: 0.0,
        required_torrent_uploads: 0,
        required_torrent_uploads_in_unique_title_groups: 0,
        required_uploaded: 0,
        required_torrent_snatched: 0,
        required_downloaded: 0,
        required_forum_posts: 0,
        required_forum_posts_in_unique_threads: 0,
        required_title_group_comments: 0,
        required_seeding_size: 0,
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes/test_class")
        .set_json(&edited)
        .to_request();

    let updated: UserClass =
        call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;
    assert_eq!(updated.name, "advanced_user");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_edit_nonexistent_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::EditUserClass).await;

    let edited = EditedUserClass {
        name: "new_name".into(),
        default_permissions: vec![],
        automatic_promotion: true,
        automatic_demotion: true,
        promotion_allowed_while_warned: false,
        required_account_age_in_days: 0,
        required_ratio: 0.0,
        required_torrent_uploads: 0,
        required_torrent_uploads_in_unique_title_groups: 0,
        required_uploaded: 0,
        required_torrent_snatched: 0,
        required_downloaded: 0,
        required_forum_posts: 0,
        required_forum_posts_in_unique_threads: 0,
        required_title_group_comments: 0,
        required_seeding_size: 0,
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes/nonexistent")
        .set_json(&edited)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// DELETE USER CLASS TESTS
// ============================================================================

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_can_delete_user_class_with_migration(pool: PgPool) {
    let pool_arc = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        Arc::clone(&pool_arc),
        MockRedisPool::default(),
        TestUser::DeleteUserClass,
    )
    .await;

    // Delete test_class and migrate users to empty_class
    let delete_body = DeleteUserClass {
        target_class_name: "empty_class".into(),
    };

    let req = test::TestRequest::delete()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes/test_class")
        .set_json(&delete_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_delete_with_nonexistent_target_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::DeleteUserClass).await;

    let delete_body = DeleteUserClass {
        target_class_name: "nonexistent".into(),
    };

    let req = test::TestRequest::delete()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes/test_class")
        .set_json(&delete_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_delete_nonexistent_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::DeleteUserClass).await;

    let delete_body = DeleteUserClass {
        target_class_name: "newbie".into(),
    };

    let req = test::TestRequest::delete()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/user-classes/nonexistent")
        .set_json(&delete_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// EDIT USER PERMISSIONS TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_staff_can_edit_user_permissions(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        TestUser::EditUserPermissions,
    )
    .await;

    let permissions = UpdatedUserPermissions {
        permissions: vec![UserPermission::EditArtist, UserPermission::EditSeries],
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/permissions")
        .set_json(&permissions)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test(
    fixtures("with_test_users", "with_locked_user"),
    migrations = "../storage/migrations"
)]
async fn test_cannot_edit_permissions_of_locked_user(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        TestUser::EditUserPermissions,
    )
    .await;

    let permissions = UpdatedUserPermissions {
        permissions: vec![UserPermission::EditArtist],
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/999/permissions")
        .set_json(&permissions)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

// ============================================================================
// LOCK USER CLASS TESTS
// ============================================================================

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_staff_can_lock_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::LockUserClass).await;

    let lock_status = UserClassLockStatus { class_locked: true };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/lock-class")
        .set_json(&lock_status)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_staff_can_unlock_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::LockUserClass).await;

    let lock_status = UserClassLockStatus {
        class_locked: false,
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/lock-class")
        .set_json(&lock_status)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

// ============================================================================
// CHANGE USER CLASS TESTS
// ============================================================================

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_change_user_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::ChangeUserClass).await;

    let class_change = UserClassChange {
        class_name: "test_class".into(),
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/class")
        .set_json(&class_change)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test(
    fixtures("with_test_users", "with_locked_user"),
    migrations = "../storage/migrations"
)]
async fn test_cannot_change_class_of_locked_user(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::ChangeUserClass).await;

    let class_change = UserClassChange {
        class_name: "newbie".into(),
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/999/class")
        .set_json(&class_change)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_user_classes"),
    migrations = "../storage/migrations"
)]
async fn test_cannot_change_to_nonexistent_class(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::ChangeUserClass).await;

    let class_change = UserClassChange {
        class_name: "nonexistent_class".into(),
    };

    let req = test::TestRequest::put()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/100/class")
        .set_json(&class_change)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
