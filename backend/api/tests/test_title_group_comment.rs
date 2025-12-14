pub mod common;
pub mod mocks;

use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::title_group_comment::{
    EditedTitleGroupComment, TitleGroupComment, UserCreatedTitleGroupComment,
};
use common::{auth_header, create_test_app_and_login, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_owner_can_edit_their_comment(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // Create comment
    let create_body = UserCreatedTitleGroupComment {
        content: "Original comment".into(),
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

    let comment: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Edit comment
    let edit_body = EditedTitleGroupComment {
        content: "Edited comment".into(),
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri(&format!("/api/title-groups/comments/{}", comment.id))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&edit_body)
        .to_request();

    let edited: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.content, "Edited comment");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_any_comment(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool.clone()));

    // User creates comment
    let (service, user) =
        create_test_app_and_login(pool.clone(), MockRedisPool::default(), TestUser::Standard).await;

    let create_body = UserCreatedTitleGroupComment {
        content: "User comment".into(),
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

    let comment: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::CREATED).await;

    // Staff edits it
    let (service, staff) = create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        TestUser::EditTitleGroupComment,
    )
    .await;

    let edit_body = EditedTitleGroupComment {
        content: "Staff edited".into(),
        locked: false,
    };

    let req = test::TestRequest::put()
        .uri(&format!("/api/title-groups/comments/{}", comment.id))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&staff.token))
        .set_json(&edit_body)
        .to_request();

    let edited: TitleGroupComment =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(edited.content, "Staff edited");
}
