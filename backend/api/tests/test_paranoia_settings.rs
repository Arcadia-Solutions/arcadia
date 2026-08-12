pub mod common;
pub mod mocks;

use actix_web::{
    dev::{Service, ServiceResponse},
    http::StatusCode,
    test,
};
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::{
        arcadia_settings::DisplayableUserStats,
        common::PaginatedResults,
        user::{
            HideableUserList, PublicProfile, UpdateUploadedTorrentsAnonymity, UserSearchResult,
            UserSettingsResponse,
        },
    },
};
use common::{auth_header, call_and_read_body_json, create_test_app_and_login, login_as, TestUser};
use mocks::mock_redis::MockRedisPool;
use sqlx::PgPool;
use std::sync::Arc;

const BASIC_USER_ID: i32 = 100;

fn get_public_profile_request(token: &str) -> actix_http::Request {
    test::TestRequest::get()
        .insert_header(auth_header(token))
        .uri(&format!("/api/users?id={BASIC_USER_ID}"))
        .to_request()
}

fn search_uploaded_torrents_request(token: &str) -> actix_http::Request {
    test::TestRequest::get()
        .insert_header(auth_header(token))
        .uri(&format!("/api/search/torrents/lite?torrent_created_by_id={BASIC_USER_ID}&page=1&page_size=5&order_by_column=torrent_created_at&order_by_direction=desc&title_group_include_empty_groups=false"))
        .to_request()
}

/// Hides the given statistics and lists with the paranoia settings of the logged in user.
async fn hide_user_information<S>(
    service: &S,
    token: &str,
    hidden_stats: Vec<DisplayableUserStats>,
    hidden_lists: Vec<HideableUserList>,
) where
    S: Service<actix_http::Request, Response = ServiceResponse, Error = actix_web::Error>,
{
    let req = test::TestRequest::get()
        .insert_header(auth_header(token))
        .uri("/api/users/settings")
        .to_request();
    let mut settings = call_and_read_body_json::<UserSettingsResponse, _>(service, req)
        .await
        .settings;
    settings.paranoia_hidden_stats = hidden_stats;
    settings.paranoia_hidden_lists = hidden_lists;

    let req = test::TestRequest::put()
        .insert_header(auth_header(token))
        .uri("/api/users/settings")
        .set_json(&settings)
        .to_request();
    assert_eq!(
        test::call_service(service, req).await.status(),
        StatusCode::OK
    );
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_paranoia_settings_hide_user_information(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, basic_user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    hide_user_information(
        &service,
        &basic_user.token,
        vec![DisplayableUserStats::ForumPosts],
        vec![HideableUserList::Torrents],
    )
    .await;

    // the user still sees everything about themselves
    let own_profile = call_and_read_body_json::<PublicProfile, _>(
        &service,
        get_public_profile_request(&basic_user.token),
    )
    .await;
    assert!(own_profile.user.forum_posts.is_some());

    // another user does not see the hidden statistic
    let other_user = login_as(&service, TestUser::EditArtist).await;
    let profile = call_and_read_body_json::<PublicProfile, _>(
        &service,
        get_public_profile_request(&other_user.token),
    )
    .await;
    assert!(profile.user.forum_posts.is_none());
    assert!(profile.user.uploaded.is_some());

    // and cannot search the hidden list either
    assert_eq!(
        test::call_service(
            &service,
            search_uploaded_torrents_request(&other_user.token)
        )
        .await
        .status(),
        StatusCode::FORBIDDEN
    );

    // a user with the dedicated permission sees everything
    let staff_user = login_as(&service, TestUser::SeeParanoiaHiddenUserInfo).await;
    let profile = call_and_read_body_json::<PublicProfile, _>(
        &service,
        get_public_profile_request(&staff_user.token),
    )
    .await;
    assert!(profile.user.forum_posts.is_some());
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_hiding_a_count_also_hides_its_list(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, basic_user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // the list itself is not hidden, only the count it belongs to
    hide_user_information(
        &service,
        &basic_user.token,
        vec![DisplayableUserStats::Snatched],
        vec![],
    )
    .await;

    let other_user = login_as(&service, TestUser::EditArtist).await;
    let req = test::TestRequest::get()
        .insert_header(auth_header(&other_user.token))
        .uri(&format!("/api/search/torrents/lite?torrent_snatched_by_id={BASIC_USER_ID}&page=1&page_size=5&order_by_column=torrent_snatched_at&order_by_direction=desc&title_group_include_empty_groups=false"))
        .to_request();
    assert_eq!(
        test::call_service(&service, req).await.status(),
        StatusCode::FORBIDDEN
    );
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrents_of_basic_user",
        "with_refreshed_title_group_hierarchy_lite"
    ),
    migrations = "../storage/migrations"
)]
async fn test_paranoia_settings_hide_the_uploaded_torrents_list(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, basic_user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    hide_user_information(
        &service,
        &basic_user.token,
        vec![],
        vec![HideableUserList::Torrents],
    )
    .await;

    // the user still sees their own uploads
    let own_profile = call_and_read_body_json::<PublicProfile, _>(
        &service,
        get_public_profile_request(&basic_user.token),
    )
    .await;
    assert!(!own_profile.last_five_uploaded_torrents.is_empty());

    // another user gets neither the list on the profile nor the torrent search
    let other_user = login_as(&service, TestUser::EditArtist).await;
    let profile = call_and_read_body_json::<PublicProfile, _>(
        &service,
        get_public_profile_request(&other_user.token),
    )
    .await;
    assert!(profile.last_five_uploaded_torrents.is_empty());
    assert_eq!(
        test::call_service(
            &service,
            search_uploaded_torrents_request(&other_user.token)
        )
        .await
        .status(),
        StatusCode::FORBIDDEN
    );

    // the dedicated permission gives access to both
    let staff_user = login_as(&service, TestUser::SeeParanoiaHiddenUserInfo).await;
    let profile = call_and_read_body_json::<PublicProfile, _>(
        &service,
        get_public_profile_request(&staff_user.token),
    )
    .await;
    assert!(!profile.last_five_uploaded_torrents.is_empty());
    assert_eq!(
        test::call_service(
            &service,
            search_uploaded_torrents_request(&staff_user.token)
        )
        .await
        .status(),
        StatusCode::OK
    );
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_paranoia_settings_hide_statistics_in_the_user_search(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, basic_user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    hide_user_information(
        &service,
        &basic_user.token,
        vec![
            DisplayableUserStats::Ratio,
            DisplayableUserStats::ForumPosts,
        ],
        vec![],
    )
    .await;

    let searching_user = login_as(&service, TestUser::SearchUsers).await;
    let req = test::TestRequest::get()
        .insert_header(auth_header(&searching_user.token))
        .uri("/api/search/users?username=user_basic&order_by=username&order_by_direction=asc&page=1&page_size=20")
        .to_request();
    let results =
        call_and_read_body_json::<PaginatedResults<UserSearchResult>, _>(&service, req).await;

    let found_user = results
        .results
        .iter()
        .find(|found_user| found_user.id == BASIC_USER_ID)
        .expect("the searched user is in the results");
    assert!(found_user.forum_posts.is_none());
    // the ratio is computed from the uploaded and downloaded amounts, hiding it hides them too
    assert!(found_user.uploaded.is_none());
    assert!(found_user.downloaded.is_none());
    assert!(found_user.seeding.is_some());
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrents_of_basic_user"
    ),
    migrations = "../storage/migrations"
)]
async fn test_updating_the_anonymity_of_all_uploaded_torrents(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, basic_user) =
        create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let get_settings_request = || {
        test::TestRequest::get()
            .insert_header(auth_header(&basic_user.token))
            .uri("/api/users/settings")
            .to_request()
    };
    let settings =
        call_and_read_body_json::<UserSettingsResponse, _>(&service, get_settings_request()).await;
    assert_eq!(settings.anonymous_uploaded_torrents, 1);
    assert_eq!(settings.non_anonymous_uploaded_torrents, 1);

    let req = test::TestRequest::put()
        .insert_header(auth_header(&basic_user.token))
        .uri("/api/users/uploaded-torrents-anonymity")
        .set_json(UpdateUploadedTorrentsAnonymity { anonymous: true })
        .to_request();
    assert_eq!(
        test::call_service(&service, req).await.status(),
        StatusCode::OK
    );

    let settings =
        call_and_read_body_json::<UserSettingsResponse, _>(&service, get_settings_request()).await;
    assert_eq!(settings.anonymous_uploaded_torrents, 2);
    assert_eq!(settings.non_anonymous_uploaded_torrents, 0);
}
