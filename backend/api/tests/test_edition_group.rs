pub mod common;
pub mod mocks;

use crate::common::TestUser;
use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::edition_group::{EditedEditionGroup, EditionGroup};
use common::auth_header;
use common::create_test_app_and_login;
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_user_with_permission_can_edit_edition_group(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::EditEditionGroup).await;

    let req_body = EditedEditionGroup {
        id: 1,
        name: Some("Updated Edition Name".into()),
        release_date: chrono::NaiveDate::from_ymd_opt(1962, 10, 5).unwrap(),
        description: Some("Updated description for the edition.".into()),
        distributor: Some("Updated Distributor".into()),
        covers: vec!["https://example.com/new-cover.jpg".into()],
        external_links: vec!["https://discogs.com/release/updated".into()],
        source: Some(arcadia_storage::models::edition_group::Source::Vinyl),
        additional_information: None,
    };

    let req = test::TestRequest::put()
        .uri("/api/edition-groups")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&req_body)
        .to_request();

    let response = common::call_and_read_body_json_with_status::<EditionGroup, _>(
        &service,
        req,
        StatusCode::OK,
    )
    .await;

    assert_eq!(response.name, req_body.name);
    assert_eq!(response.description, req_body.description);
    assert_eq!(response.distributor, req_body.distributor);
    assert_eq!(response.release_date, req_body.release_date);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_user_without_permission_cannot_edit_edition_group(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req_body = EditedEditionGroup {
        id: 1,
        name: Some("Should Not Update".into()),
        release_date: chrono::NaiveDate::from_ymd_opt(1962, 10, 5).unwrap(),
        description: Some("This should fail.".into()),
        distributor: Some("Test Distributor".into()),
        covers: vec![],
        external_links: vec![],
        source: None,
        additional_information: None,
    };

    let req = test::TestRequest::put()
        .uri("/api/edition-groups")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&service, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
