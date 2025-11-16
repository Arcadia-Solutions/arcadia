pub mod common;
pub mod mocks;

use crate::common::TestUser;
use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::artist::{Artist, UserEditedArtist};
use common::auth_header;
use common::create_test_app_and_login;
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_user2", "with_test_artist"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_artist(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), 100, 100, TestUser::Staff).await;

    let req_body = UserEditedArtist{
        id: 1,
        name: "Beatles, The".into(),
        description: "They are actually called 'The Beatles', but we decided to be weird with articles.".into(),
        pictures: vec![
            "https://upload.wikimedia.org/wikipedia/commons/d/d8/The_Beatles_members_at_New_York_City_in_1964.jpg".into()
        ],
    };

    let req = test::TestRequest::put()
        .uri("/api/artists")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&req_body)
        .to_request();

    let response =
        common::call_and_read_body_json_with_status::<Artist, _>(&service, req, StatusCode::OK)
            .await;

    assert_eq!(response.name, req_body.name);
    assert_eq!(response.description, req_body.description);
    assert_eq!(response.pictures.len(), 1);
    assert_eq!(response.pictures[0], req_body.pictures[0]);
}
