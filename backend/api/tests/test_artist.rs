pub mod common;
pub mod mocks;

use crate::common::TestUser;
use actix_web::http::StatusCode;
use actix_web::test;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::artist::{Artist, ArtistSearchResult, EditedArtist};
use arcadia_storage::models::common::PaginatedResults;
use common::auth_header;
use common::create_test_app_and_login;
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

#[sqlx::test(
    fixtures("with_test_users", "with_test_artist"),
    migrations = "../storage/migrations"
)]
async fn test_staff_can_edit_artist(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::EditArtist).await;

    let req_body = EditedArtist{
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

#[sqlx::test(
    fixtures("with_test_users", "with_test_artists_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_artists_returns_paginated_results(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/artists?page=1&page_size=10")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: PaginatedResults<ArtistSearchResult> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.results.len(), 3);
    assert_eq!(response.total_items, 3);
    assert_eq!(response.page, 1);
    assert_eq!(response.page_size, 10);
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_artists_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_artists_filters_by_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/artists?name=Beatles&page=1&page_size=10")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: PaginatedResults<ArtistSearchResult> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.results.len(), 1);
    assert_eq!(response.total_items, 1);
    assert_eq!(response.results[0].name, "The Beatles");
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_artists_for_search"),
    migrations = "../storage/migrations"
)]
async fn test_search_artists_pagination(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/search/artists?page=1&page_size=2")
        .insert_header(auth_header(&user.token))
        .to_request();

    let response: PaginatedResults<ArtistSearchResult> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response.results.len(), 2);
    assert_eq!(response.total_items, 3);
    assert_eq!(response.page, 1);
    assert_eq!(response.page_size, 2);
}
