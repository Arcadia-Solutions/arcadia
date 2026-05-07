pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;

use arcadia_storage::models::forum::{ForumPostHierarchy, UserCreatedForumPostReaction};
use arcadia_storage::{connection_pool::ConnectionPool, models::common::PaginatedResults};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

// ============================================================================
// GET FORUM POST REACTIONS TESTS
// ============================================================================
#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post",
        "with_test_forum_reaction",
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_forum_post_with_one_reaction(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // We retrieve the thread
    let req = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=100&page_size=10")
        .insert_header(auth_header(&user.token))
        .to_request();

    let posts: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    // We got 2 posts for this thread from this user
    // The first post has a reaction, the second none
    assert_eq!(posts.total_items, 2);
    assert_eq!(posts.results[0].created_by.id, 100);
    assert!(posts.results[0].reaction.is_some());
    assert_eq!(posts.results[0].reaction.as_ref().unwrap().emoji, "🥰");
    assert_eq!(
        posts.results[0].reaction.as_ref().unwrap().forum_post_id,
        100
    );
    assert!(posts.results[1].reaction.is_none());
}

// ============================================================================
// POST FORUM POST REACTIONS TESTS
// ============================================================================
#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post",
        "with_test_forum_reaction"
    ),
    migrations = "../storage/migrations"
)]
async fn test_post_forum_post_reaction(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // We get the post and check if not reaction is bound to it
    let req_get = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=103&page_size=10")
        .insert_header(auth_header(&user.token))
        .to_request();

    let posts: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req_get, StatusCode::OK).await;

    assert_eq!(posts.total_items, 1);
    assert_eq!(posts.results[0].id, 103);
    assert!(posts.results[0].reaction.is_none());

    // We create the reaction and test that it was created
    let create_body = UserCreatedForumPostReaction {
        emoji: "😺".to_string(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/post/103/reaction")
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let post: ForumPostHierarchy =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(post.id, 103);
    assert_eq!(post.reaction.as_ref().unwrap().forum_post_id, 103);
    assert_eq!(post.reaction.as_ref().unwrap().emoji, "😺");
}

// ============================================================================
// PUT FORUM POST REACTIONS TESTS
// ============================================================================
#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post",
        "with_test_forum_reaction",
    ),
    migrations = "../storage/migrations"
)]
async fn test_put_forum_post_reaction(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // We retrieve the old reaction to check it's already created and has the emoji 🥰
    let req_get = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=100&page_size=10")
        .insert_header(auth_header(&user.token))
        .to_request();

    let posts: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req_get, StatusCode::OK).await;

    assert!(posts.results[0].reaction.is_some());
    assert_eq!(posts.results[0].id, 100);
    assert_eq!(posts.results[0].reaction.as_ref().unwrap().emoji, "🥰");

    // We modify the reaction (the emoji)
    let create_body = UserCreatedForumPostReaction {
        emoji: "😺".to_string(),
    };

    let req = test::TestRequest::put()
        .uri("/api/forum/post/100/reaction")
        .insert_header(auth_header(&user.token))
        .set_json(&create_body)
        .to_request();

    let post: ForumPostHierarchy =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(post.reaction.as_ref().unwrap().id, 100);
    assert_eq!(post.reaction.as_ref().unwrap().emoji, "😺");
}

// ============================================================================
// DELETE FORUM POST REACTIONS TESTS
// ============================================================================
#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_forum_category",
        "with_test_forum_sub_category",
        "with_test_forum_thread",
        "with_test_forum_post",
        "with_test_forum_reaction",
    ),
    migrations = "../storage/migrations"
)]
async fn test_delete_forum_post_reaction(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::delete()
        .uri("/api/forum/post/100/reaction")
        .insert_header(auth_header(&user.token))
        .to_request();

    let res = test::call_service(&service, req).await;
    assert_eq!(res.status(), StatusCode::OK);

    // Verify delete by getting the post and checking that reaction is null
    let req_get = test::TestRequest::get()
        .uri("/api/forum/thread/posts?thread_id=100&page_size=10&post_id=100")
        .insert_header(auth_header(&user.token))
        .to_request();

    let posts: PaginatedResults<ForumPostHierarchy> =
        common::call_and_read_body_json_with_status(&service, req_get, StatusCode::OK).await;

    assert!(posts.results[0].reaction.is_none());
}
