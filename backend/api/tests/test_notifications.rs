pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::{
    forum::{ForumPost, UserCreatedForumPost},
    notification::{
        NotificationForumThreadPost, NotificationStaffPmMessage, NotificationTitleGroupComment,
    },
    staff_pm::{StaffPm, StaffPmMessage, UserCreatedStaffPm, UserCreatedStaffPmMessage},
    title_group_comment::{TitleGroupComment, UserCreatedTitleGroupComment},
    user::Profile,
};
use common::{auth_header, create_test_app, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

// Title Group Comment Notifications

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_subscriber_receives_notification_on_new_title_group_comment(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A (Standard) will create comments
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B (EditTitleGroupComment) will subscribe and receive notifications
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_edit_tgc",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    // User B subscribes to title group 1
    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/title-group-comments?title_group_id=1")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let resp = test::call_service(&service_b, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // User A creates a comment on title group 1
    let create_body = UserCreatedTitleGroupComment {
        content: "Test comment for notification".into(),
        title_group_id: 1,
        refers_to_torrent_id: None,
        answers_to_comment_id: None,
    };
    let req = test::TestRequest::post()
        .uri("/api/title-groups/comments")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let _comment: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User B checks notifications
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/title-group-comments?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationTitleGroupComment> =
        common::call_and_read_body_json(&service_b, notif_req).await;

    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].title_group_id, 1);
    assert!(!notifications[0].read_status);

    // Verify counter in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_title_group_comments, 1);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_comment_creator_does_not_receive_own_notification(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    let (service, user) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User subscribes to title group 1
    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/title-group-comments?title_group_id=1")
        .insert_header(auth_header(&user.token))
        .to_request();
    let resp = test::call_service(&service, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Same user creates a comment
    let create_body = UserCreatedTitleGroupComment {
        content: "My own comment".into(),
        title_group_id: 1,
        refers_to_torrent_id: None,
        answers_to_comment_id: None,
    };
    let req = test::TestRequest::post()
        .uri("/api/title-groups/comments")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();
    let _comment: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User should NOT receive notification for their own comment
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/title-group-comments?include_read=false")
        .insert_header(auth_header(&user.token))
        .to_request();
    let notifications: Vec<NotificationTitleGroupComment> =
        common::call_and_read_body_json(&service, notif_req).await;

    assert_eq!(notifications.len(), 0);

    // Verify counter stays at 0 in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service, me_req).await;
    assert_eq!(profile.unread_notifications_amount_title_group_comments, 0);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_no_duplicate_unread_title_group_notifications(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A creates comments
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B subscribes
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_edit_tgc",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/title-group-comments?title_group_id=1")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let resp = test::call_service(&service_b, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // User A creates two comments
    for i in 1..=2 {
        let create_body = UserCreatedTitleGroupComment {
            content: format!("Comment {}", i),
            title_group_id: 1,
            refers_to_torrent_id: None,
            answers_to_comment_id: None,
        };
        let req = test::TestRequest::post()
            .uri("/api/title-groups/comments")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .insert_header(auth_header(&user_a.token))
            .set_json(&create_body)
            .to_request();
        let _: TitleGroupComment =
            common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    }

    // User B should only have 1 unread notification (no duplicates)
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/title-group-comments?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationTitleGroupComment> =
        common::call_and_read_body_json(&service_b, notif_req).await;

    assert_eq!(notifications.len(), 1);

    // Verify counter is 1 (not 2) in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_title_group_comments, 1);
}

// Forum Thread Post Notifications

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread"
    ),
    migrations = "../storage/migrations"
)]
async fn test_subscriber_receives_notification_on_new_forum_post(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A creates posts
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B subscribes and receives notifications
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_edit_tgc",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    // User B subscribes to forum thread 100
    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/forum-thread-posts?thread_id=100")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let resp = test::call_service(&service_b, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // User A creates a post in thread 100
    let create_body = UserCreatedForumPost {
        content: "Test post for notification".into(),
        forum_thread_id: 100,
    };
    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let _post: ForumPost =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User B checks notifications
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/forum-thread-posts?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationForumThreadPost> =
        common::call_and_read_body_json(&service_b, notif_req).await;

    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].forum_thread_id, 100);
    assert!(!notifications[0].read_status);

    // Verify counter in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_forum_thread_posts, 1);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread"
    ),
    migrations = "../storage/migrations"
)]
async fn test_post_creator_does_not_receive_own_notification(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    let (service, user) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User subscribes to thread 100
    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/forum-thread-posts?thread_id=100")
        .insert_header(auth_header(&user.token))
        .to_request();
    let resp = test::call_service(&service, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Same user creates a post
    let create_body = UserCreatedForumPost {
        content: "My own post".into(),
        forum_thread_id: 100,
    };
    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();
    let _post: ForumPost =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User should NOT receive notification for their own post
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/forum-thread-posts?include_read=false")
        .insert_header(auth_header(&user.token))
        .to_request();
    let notifications: Vec<NotificationForumThreadPost> =
        common::call_and_read_body_json(&service, notif_req).await;

    assert_eq!(notifications.len(), 0);

    // Verify counter stays at 0 in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service, me_req).await;
    assert_eq!(profile.unread_notifications_amount_forum_thread_posts, 0);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread"
    ),
    migrations = "../storage/migrations"
)]
async fn test_no_duplicate_unread_forum_notifications(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A creates posts
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B subscribes
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_edit_tgc",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/forum-thread-posts?thread_id=100")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let resp = test::call_service(&service_b, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // User A creates two posts
    for i in 1..=2 {
        let create_body = UserCreatedForumPost {
            content: format!("Post {}", i),
            forum_thread_id: 100,
        };
        let req = test::TestRequest::post()
            .uri("/api/forum/post")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .insert_header(auth_header(&user_a.token))
            .set_json(&create_body)
            .to_request();
        let _: ForumPost =
            common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;
    }

    // User B should only have 1 unread notification (no duplicates)
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/forum-thread-posts?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationForumThreadPost> =
        common::call_and_read_body_json(&service_b, notif_req).await;

    assert_eq!(notifications.len(), 1);

    // Verify counter is 1 (not 2) in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_forum_thread_posts, 1);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_include_read_filter_title_group_notifications(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A creates comments
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B subscribes
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_edit_tgc",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    let sub_req = test::TestRequest::post()
        .uri("/api/subscriptions/title-group-comments?title_group_id=1")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let resp = test::call_service(&service_b, sub_req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // User A creates a comment
    let create_body = UserCreatedTitleGroupComment {
        content: "Comment to mark as read".into(),
        title_group_id: 1,
        refers_to_torrent_id: None,
        answers_to_comment_id: None,
    };
    let req = test::TestRequest::post()
        .uri("/api/title-groups/comments")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let _: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // With include_read=false, should see 1 notification
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/title-group-comments?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationTitleGroupComment> =
        common::call_and_read_body_json(&service_b, notif_req).await;
    assert_eq!(notifications.len(), 1);

    // Verify counter in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_title_group_comments, 1);

    // With include_read=true, should also see 1 notification (same result, just includes read ones too)
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/title-group-comments?include_read=true")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationTitleGroupComment> =
        common::call_and_read_body_json(&service_b, notif_req).await;
    assert_eq!(notifications.len(), 1);
}

// Conversation Notifications

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_conversation_counter_increments_for_receiver(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A (Standard, id=100) will create a conversation
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B (EditTitleGroupComment, id=103) will receive the conversation
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_edit_tgc",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    // User B should have 0 unread conversations initially
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_conversations_amount, 0);

    // User A creates a conversation with User B (id=103)
    let req = test::TestRequest::post()
        .uri("/api/conversations")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(serde_json::json!({
            "subject": "Test conversation",
            "receiver_id": 103,
            "first_message": {
                "conversation_id": 0,
                "content": "Hello, this is a test message"
            }
        }))
        .to_request();
    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // User B should now have 1 unread conversation
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_conversations_amount, 1);
}

// Staff PM Notifications

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_staff_receives_notification_on_new_staff_pm(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A (Standard) creates a staff PM
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B (Staff with ReadStaffPm permission, id=138) should receive notification
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_staff_pm",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    // User A creates a staff PM
    let create_body = UserCreatedStaffPm {
        subject: "Test Staff PM".into(),
        first_message: UserCreatedStaffPmMessage {
            staff_pm_id: 0,
            content: "This is a test staff PM message".into(),
        },
    };
    let req = test::TestRequest::post()
        .uri("/api/staff-pms")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let _staff_pm: StaffPm =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User B (staff) checks notifications
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service_b, notif_req).await;

    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].staff_pm_subject, "Test Staff PM");
    assert!(!notifications[0].read_status);

    // Verify counter in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_staff_pm_messages, 1);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_creator_receives_notification_on_staff_reply(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A (Standard) creates a staff PM
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B (Staff) will reply
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_staff_pm",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    // User A creates a staff PM
    let create_body = UserCreatedStaffPm {
        subject: "Test Staff PM for reply".into(),
        first_message: UserCreatedStaffPmMessage {
            staff_pm_id: 0,
            content: "Initial message".into(),
        },
    };
    let req = test::TestRequest::post()
        .uri("/api/staff-pms")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let staff_pm: StaffPm =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Clear user A's notifications (they created the PM so shouldn't have any)
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=false")
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service, notif_req).await;
    assert_eq!(notifications.len(), 0);

    // User B (staff) replies to the staff PM
    let reply_body = UserCreatedStaffPmMessage {
        staff_pm_id: staff_pm.id,
        content: "Staff reply".into(),
    };
    let req = test::TestRequest::post()
        .uri("/api/staff-pms/messages")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_b.token))
        .set_json(&reply_body)
        .to_request();
    let _reply: StaffPmMessage =
        common::call_and_read_body_json_with_status(&service_b, req, StatusCode::CREATED).await;

    // User A (creator) should now have a notification
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=false")
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service, notif_req).await;

    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].staff_pm_id, staff_pm.id);

    // Verify counter in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service, me_req).await;
    assert_eq!(profile.unread_notifications_amount_staff_pm_messages, 1);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_message_creator_does_not_receive_own_staff_pm_notification(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A (Standard) creates a staff PM
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User A creates a staff PM
    let create_body = UserCreatedStaffPm {
        subject: "Test Staff PM".into(),
        first_message: UserCreatedStaffPmMessage {
            staff_pm_id: 0,
            content: "My own message".into(),
        },
    };
    let req = test::TestRequest::post()
        .uri("/api/staff-pms")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let _staff_pm: StaffPm =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User A should NOT receive notification for their own message
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=false")
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service, notif_req).await;

    assert_eq!(notifications.len(), 0);

    // Verify counter stays at 0 in get me route
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service, me_req).await;
    assert_eq!(profile.unread_notifications_amount_staff_pm_messages, 0);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_notifications_marked_as_read_when_staff_pm_resolved(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A (Standard) creates a staff PM
    let (service, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    // User B (Staff) will resolve the PM
    let mock_redis = MockRedisPool::default();
    let service_b = create_test_app(pool.clone(), mock_redis).await;
    let login_req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "username": "user_staff_pm",
            "password": "test_password",
            "remember_me": true
        }))
        .to_request();
    let user_b: arcadia_storage::models::user::LoginResponse =
        common::call_and_read_body_json(&service_b, login_req).await;

    // User A creates a staff PM
    let create_body = UserCreatedStaffPm {
        subject: "Staff PM to resolve".into(),
        first_message: UserCreatedStaffPmMessage {
            staff_pm_id: 0,
            content: "Please help".into(),
        },
    };
    let req = test::TestRequest::post()
        .uri("/api/staff-pms")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .set_json(&create_body)
        .to_request();
    let staff_pm: StaffPm =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // User B (staff) should have unread notification
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service_b, notif_req).await;
    assert_eq!(notifications.len(), 1);

    // Verify counter in get me route before resolve
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_staff_pm_messages, 1);

    // User B resolves the staff PM
    let resolve_req = test::TestRequest::put()
        .uri(&format!("/api/staff-pms/{}/resolve", staff_pm.id))
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let resp = test::call_service(&service_b, resolve_req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // User B should have no unread notifications now
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=false")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service_b, notif_req).await;
    assert_eq!(notifications.len(), 0);

    // Verify counter in get me route after resolve
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_b, me_req).await;
    assert_eq!(profile.unread_notifications_amount_staff_pm_messages, 0);

    // But with include_read=true, should still see it
    let notif_req = test::TestRequest::get()
        .uri("/api/notifications/staff-pm-messages?include_read=true")
        .insert_header(auth_header(&user_b.token))
        .to_request();
    let notifications: Vec<NotificationStaffPmMessage> =
        common::call_and_read_body_json(&service_b, notif_req).await;
    assert_eq!(notifications.len(), 1);
    assert!(notifications[0].read_status);
}

// Unread Announcements

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_unread_announcements_counts_unseen_threads_in_subcategory_1(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // The initdb creates one thread (id=1) in subcategory 1 (Announcements).
    // A fresh user has never viewed it, so unread_announcements_amount should be 1.
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service, me_req).await;
    assert_eq!(profile.unread_announcements_amount, 1);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_unread_announcements_decreases_after_viewing_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // View the announcement thread's posts (thread id=1), which creates a read marker
    let req = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=1&page_size=10")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();
    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // After viewing, the announcement should be counted as read
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service, me_req).await;
    assert_eq!(profile.unread_announcements_amount, 0);
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_unread_announcements_stays_read_after_new_post_in_thread(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // User A views the announcement
    let (service_a, user_a) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=1&page_size=10")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let resp = test::call_service(&service_a, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // User B posts a new reply in the announcement thread
    let (service_b, user_b) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::EditArtist)
            .await;

    let post_body = UserCreatedForumPost {
        content: "New reply in announcement".into(),
        forum_thread_id: 1,
    };
    let req = test::TestRequest::post()
        .uri("/api/forum/post")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user_b.token))
        .set_json(&post_body)
        .to_request();
    let _: ForumPost =
        common::call_and_read_body_json_with_status(&service_b, req, StatusCode::CREATED).await;

    // User A's unread_announcements_amount should still be 0 because the thread was seen once
    let me_req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(auth_header(&user_a.token))
        .to_request();
    let profile: Profile = common::call_and_read_body_json(&service_a, me_req).await;
    assert_eq!(profile.unread_announcements_amount, 0);
}
