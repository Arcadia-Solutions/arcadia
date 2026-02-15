pub mod common;
pub mod mocks;

use std::sync::Arc;

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    http::StatusCode,
    test, Error,
};
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::{
        common::{OrderByDirection, PaginatedResults},
        peer::PublicPeer,
        title_group::TitleGroupHierarchyLite,
        torrent::{TorrentSearch, TorrentSearchOrderByColumn},
    },
};
use mocks::mock_redis::MockRedisPool;
use serde::{de::DeserializeOwned, Deserialize};
use sqlx::PgPool;

use crate::common::{auth_header, TestUser};

async fn upload_test_torrent<S, T>(service: &S, token: &str, release_group: &str) -> T
where
    S: Service<Request, Response = ServiceResponse, Error = Error>,
    T: DeserializeOwned,
{
    use actix_multipart_rfc7578::client::multipart;

    let mut form = multipart::Form::default();

    form.add_text("release_name", "test release name");
    form.add_text("release_group", release_group);
    form.add_text("description", "This is a test description");
    form.add_text("uploaded_as_anonymous", "true");
    form.add_text("mediainfo", "test mediainfo");
    form.add_text("languages", "English");
    form.add_text("container", "MKV");
    form.add_text("edition_group_id", "1");
    form.add_text("duration", "3600");
    form.add_text("audio_codec", "flac");
    form.add_text("audio_bitrate", "1200");
    form.add_text("audio_channels", "5.1");
    form.add_text("audio_bitrate_sampling", "256");
    form.add_text("video_codec", "h264");
    form.add_text("features", "DV,HDR");
    form.add_text("subtitle_languages", "English,French");
    form.add_text("video_resolution", "1080p");
    form.add_text("extras", "");
    form.add_text("bonus_points_snatch_cost", "0");

    let torrent_data = bytes::Bytes::from_static(include_bytes!(
        "data/debian-12.10.0-i386-netinst.iso.torrent"
    ));

    form.add_reader_file(
        "torrent_file",
        std::io::Cursor::new(torrent_data),
        "torrent_file.torrent",
    );

    let content_type = form.content_type();

    let payload = actix_web::body::to_bytes(multipart::Body::from(form))
        .await
        .unwrap();

    let req = test::TestRequest::post()
        .uri("/api/torrents")
        .insert_header(auth_header(token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Content-Type", content_type))
        .set_payload(payload)
        .to_request();

    common::call_and_read_body_json_with_status::<T, _>(service, req, StatusCode::CREATED).await
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_valid_torrent(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/torrents?id=1")
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    // A minimum set of definitions for assertions.
    #[derive(Debug, Deserialize)]
    struct Info {
        private: isize,
    }

    #[derive(Debug, Deserialize)]
    struct MetaInfo {
        info: Info,
        announce: String,
    }

    let metainfo = common::read_body_bencode::<MetaInfo, _>(resp)
        .await
        .expect("could not deserialize metainfo");

    assert_eq!(
        metainfo.info.private, 1,
        "expected downloaded torrent to be private"
    );

    let test_user_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    assert!(
        metainfo.announce.contains(test_user_passkey),
        "expected announce url to contain test_user passkey"
    );
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_upload_torrent(pool: PgPool) {
    let pg_pool = pool.clone();

    // Set bonus points given on upload BEFORE creating the app
    // (settings are loaded into memory at app creation time)
    let bonus_points_on_upload: i64 = 500;
    sqlx::query("UPDATE arcadia_settings SET bonus_points_given_on_upload = $1")
        .bind(bonus_points_on_upload)
        .execute(&pg_pool)
        .await
        .unwrap();

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    // Get user's initial bonus points
    let initial_bonus_points: i64 =
        sqlx::query_scalar("SELECT bonus_points FROM users WHERE id = 100")
            .fetch_one(&pg_pool)
            .await
            .unwrap();

    #[derive(Debug, Deserialize)]
    struct Torrent {
        edition_group_id: i64,
        created_by_id: i32,
    }

    let torrent: Torrent = upload_test_torrent(&service, &user.token, "TESTGRoUP").await;

    assert_eq!(torrent.edition_group_id, 1);
    assert_eq!(torrent.created_by_id, 100);

    // Verify bonus points were awarded
    let final_bonus_points: i64 =
        sqlx::query_scalar("SELECT bonus_points FROM users WHERE id = 100")
            .fetch_one(&pg_pool)
            .await
            .unwrap();

    assert_eq!(
        final_bonus_points,
        initial_bonus_points + bonus_points_on_upload,
        "expected user to receive {} bonus points for uploading",
        bonus_points_on_upload
    );
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_fill_torrent_request_uploader_only_within_first_hour(pool: PgPool) {
    #[derive(Debug, Deserialize)]
    struct CreatedTorrent {
        id: i32,
        created_by_id: i32,
    }

    #[derive(Debug, Deserialize)]
    struct CreatedTorrentRequest {
        id: i64,
    }

    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        error: String,
    }

    #[derive(Debug, Deserialize)]
    struct TorrentRequestAndAssociatedData {
        torrent_request: TorrentRequest,
    }

    #[derive(Debug, Deserialize)]
    struct TorrentRequest {
        filled_by_user_id: Option<i32>,
        filled_by_torrent_id: Option<i32>,
        filled_at: Option<String>,
    }

    let pg_pool = pool.clone();
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    let (service_uploader, uploader) = common::create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::Standard,
    )
    .await;

    let (service_other_user, other_user) = common::create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::EditArtist,
    )
    .await;

    // Upload torrent as uploader.
    let uploaded_torrent: CreatedTorrent =
        upload_test_torrent(&service_uploader, &uploader.token, "TESTGROUP").await;

    assert_eq!(uploaded_torrent.created_by_id, 100);

    // Give the other user some upload so the bounty can be placed.
    sqlx::query("UPDATE users SET uploaded = 10000000 WHERE id = 101")
        .execute(&pg_pool)
        .await
        .unwrap();

    // Create a torrent request (in same title group) as the other user.
    let create_request_payload = serde_json::json!({
        "title_group_id": 1,
        "edition_name": null,
        "release_group": null,
        "description": "test request",
        "languages": [],
        "container": [],
        "source": [],
        "audio_codec": [],
        "audio_channels": [],
        "audio_bitrate_sampling": [],
        "video_codec": [],
        "features": [],
        "subtitle_languages": [],
        "video_resolution": [],
        "video_resolution_other_x": null,
        "video_resolution_other_y": null,
        "initial_vote": {
            "torrent_request_id": 0,
            "bounty_upload": 1048576,
            "bounty_bonus_points": 0
        }
    });

    let req = test::TestRequest::post()
        .uri("/api/torrent-requests")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&other_user.token))
        .set_json(create_request_payload.clone())
        .to_request();

    let created_request_uploader_fill: CreatedTorrentRequest =
        common::call_and_read_body_json_with_status(&service_other_user, req, StatusCode::CREATED)
            .await;

    // Uploader can fill within the first hour.
    let req = test::TestRequest::post()
        .uri("/api/torrent-requests/fill")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&uploader.token))
        .set_json(serde_json::json!({
            "torrent_request_id": created_request_uploader_fill.id,
            "torrent_id": uploaded_torrent.id
        }))
        .to_request();

    let resp = test::call_service(&service_uploader, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!(
            "/api/torrent-requests?id={}",
            created_request_uploader_fill.id
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&uploader.token))
        .to_request();

    let request_data: TorrentRequestAndAssociatedData =
        common::call_and_read_body_json_with_status(&service_uploader, req, StatusCode::OK).await;

    assert_eq!(
        request_data.torrent_request.filled_by_user_id,
        Some(100),
        "expected request to be filled by uploader"
    );
    assert_eq!(
        request_data.torrent_request.filled_by_torrent_id,
        Some(uploaded_torrent.id),
        "expected request to be filled by uploaded torrent"
    );
    assert!(
        request_data.torrent_request.filled_at.is_some(),
        "expected request to have filled_at timestamp"
    );

    // Create another torrent request to validate the non-uploader grace period behavior.
    let req = test::TestRequest::post()
        .uri("/api/torrent-requests")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&other_user.token))
        .set_json(create_request_payload)
        .to_request();

    let created_request = common::call_and_read_body_json_with_status::<CreatedTorrentRequest, _>(
        &service_other_user,
        req,
        StatusCode::CREATED,
    )
    .await;

    // Non-uploader cannot fill within 1 hour.
    let req = test::TestRequest::post()
        .uri("/api/torrent-requests/fill")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&other_user.token))
        .set_json(serde_json::json!({
            "torrent_request_id": created_request.id,
            "torrent_id": uploaded_torrent.id
        }))
        .to_request();

    let error = common::call_and_read_body_json_with_status::<ErrorResponse, _>(
        &service_other_user,
        req,
        StatusCode::CONFLICT,
    )
    .await;

    assert!(
        error
            .error
            .contains("only the torrent uploader can fill requests"),
        "expected grace period error message, got: {:?}",
        error
    );

    // Move torrent upload time into the past so the grace period has elapsed.
    sqlx::query(
        r#"
        UPDATE torrents
        SET created_at = NOW() - INTERVAL '2 hours'
        WHERE id = $1
        "#,
    )
    .bind(uploaded_torrent.id)
    .execute(&pg_pool)
    .await
    .unwrap();

    // Non-uploader can fill after the grace period.
    let req = test::TestRequest::post()
        .uri("/api/torrent-requests/fill")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&other_user.token))
        .set_json(serde_json::json!({
            "torrent_request_id": created_request.id,
            "torrent_id": uploaded_torrent.id
        }))
        .to_request();

    let resp = test::call_service(&service_other_user, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Verify request is filled.
    let req = test::TestRequest::get()
        .uri(&format!("/api/torrent-requests?id={}", created_request.id))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&other_user.token))
        .to_request();

    let request_data: TorrentRequestAndAssociatedData =
        common::call_and_read_body_json_with_status(&service_other_user, req, StatusCode::OK).await;

    assert_eq!(
        request_data.torrent_request.filled_by_user_id,
        Some(101),
        "expected request to be filled by other user"
    );
    assert_eq!(
        request_data.torrent_request.filled_by_torrent_id,
        Some(uploaded_torrent.id),
        "expected request to be filled by uploaded torrent"
    );
    assert!(
        request_data.torrent_request.filled_at.is_some(),
        "expected request to have filled_at timestamp"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_find_torrents_by_external_link(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let query = TorrentSearch {
        title_group_name: Some("https://en.wikipedia.org/wiki/RollerCoaster_Tycoon".to_string()),
        title_group_content_type: vec![],
        title_group_category: vec![],
        title_group_include_empty_groups: true,
        edition_group_source: vec![],
        torrent_video_resolution: vec![],
        torrent_language: vec![],
        torrent_reported: None,
        torrent_staff_checked: None,
        torrent_created_by_id: None,
        torrent_snatched_by_id: None,
        artist_id: None,
        collage_id: None,
        page: 1,
        page_size: 50,
        order_by_column: TorrentSearchOrderByColumn::TorrentCreatedAt,
        order_by_direction: OrderByDirection::Desc,
        series_id: None,
    };

    let query = serde_qs::to_string(&query).unwrap();
    let uri = format!("/api/search/torrents/lite?{}", query);

    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let results: PaginatedResults<TitleGroupHierarchyLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let groups = results.results;
    assert!(
        groups
            .iter()
            .any(|g| g.id == 2 && g.name == "RollerCoaster Tycoon"),
        "expected results to include title_group id=2 (RollerCoaster Tycoon) when searching by external link"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_find_torrents_by_name(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let query = TorrentSearch {
        title_group_name: Some("Love Me Do".to_string()),
        title_group_content_type: vec![],
        title_group_category: vec![],
        title_group_include_empty_groups: true,
        edition_group_source: vec![],
        torrent_video_resolution: vec![],
        torrent_language: vec![],
        torrent_reported: None,
        torrent_staff_checked: None,
        torrent_created_by_id: None,
        torrent_snatched_by_id: None,
        artist_id: None,
        collage_id: None,
        page: 1,
        page_size: 50,
        order_by_column: TorrentSearchOrderByColumn::TorrentCreatedAt,
        order_by_direction: OrderByDirection::Desc,
        series_id: None,
    };

    let query = serde_qs::to_string(&query).unwrap();
    let uri = format!("/api/search/torrents/lite?{}", query);

    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let results: PaginatedResults<TitleGroupHierarchyLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let groups = results.results;
    assert!(
        groups
            .iter()
            .any(|g| g.id == 1 && g.name == "Love Me Do / P.S. I Love You"),
        "expected results to include title_group id=1 (Love Me Do / P.S. I Love You) when searching by name"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_find_torrents_no_link_or_name_provided(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let query = TorrentSearch {
        title_group_name: Some("".to_string()),
        title_group_content_type: vec![],
        title_group_category: vec![],
        title_group_include_empty_groups: true,
        edition_group_source: vec![],
        torrent_video_resolution: vec![],
        torrent_language: vec![],
        torrent_reported: None,
        torrent_staff_checked: None,
        torrent_created_by_id: None,
        torrent_snatched_by_id: None,
        artist_id: None,
        collage_id: None,
        page: 1,
        page_size: 50,
        order_by_column: TorrentSearchOrderByColumn::TorrentCreatedAt,
        order_by_direction: OrderByDirection::Desc,
        series_id: None,
    };

    let query = serde_qs::to_string(&query).unwrap();
    let uri = format!("/api/search/torrents/lite?{}", query);

    let req = test::TestRequest::get()
        .uri(&uri)
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    let results: PaginatedResults<TitleGroupHierarchyLite> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let groups = results.results;
    let ids: Vec<i32> = groups.iter().map(|g| g.id).collect();

    assert!(
        ids.contains(&1) && ids.contains(&2),
        "expected unfiltered results to include both title_group id=1 and id=2"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_set_torrent_staff_checked_with_permission(pool: PgPool) {
    use serde_json::json;

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = common::create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::SetTorrentStaffChecked,
    )
    .await;

    let payload = json!({
        "torrent_id": 1,
        "staff_checked": true
    });

    let req = test::TestRequest::put()
        .uri("/api/torrents/staff-checked")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&payload)
        .to_request();

    let response: serde_json::Value =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    assert_eq!(response["result"], "success");

    // Verify the torrent's staff_checked field was updated
    let torrent = pool.find_torrent(1).await.unwrap();
    assert!(torrent.staff_checked);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_set_torrent_staff_checked_without_permission(pool: PgPool) {
    use serde_json::json;

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let payload = json!({
        "torrent_id": 1,
        "staff_checked": true
    });

    let req = test::TestRequest::put()
        .uri("/api/torrents/staff-checked")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&payload)
        .to_request();

    common::call_and_read_body_json_with_status::<serde_json::Value, _>(
        &service,
        req,
        StatusCode::FORBIDDEN,
    )
    .await;
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_peers"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_torrent_peers_with_permission(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = common::create_test_app_and_login(
        pool,
        MockRedisPool::default(),
        TestUser::ViewTorrentPeers,
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/torrents/peers?torrent_id=1")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let peers: Vec<PublicPeer> =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    // Should return 3 active peers
    assert_eq!(peers.len(), 3, "expected 3 active peers");

    // Own peer (user_id 139) should have IP and port visible
    let own_peer = peers.iter().find(|p| p.user.id == 139).unwrap();
    assert!(own_peer.ip.is_some(), "own peer should have IP visible");
    assert!(own_peer.port.is_some(), "own peer should have port visible");

    // Other peers should have IP and port hidden
    let other_peer = peers.iter().find(|p| p.user.id == 100).unwrap();
    assert!(other_peer.ip.is_none(), "other peer should have IP hidden");
    assert!(
        other_peer.port.is_none(),
        "other peer should have port hidden"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_peers"
    ),
    migrations = "../storage/migrations"
)]
async fn test_get_torrent_peers_without_permission(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let req = test::TestRequest::get()
        .uri("/api/torrents/peers?torrent_id=1")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    common::call_and_read_body_json_with_status::<serde_json::Value, _>(
        &service,
        req,
        StatusCode::FORBIDDEN,
    )
    .await;
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_torrent_up_down_factors_with_permission(pool: PgPool) {
    use serde_json::json;

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) = common::create_test_app_and_login(
        pool.clone(),
        MockRedisPool::default(),
        TestUser::EditTorrentUpDownFactors,
    )
    .await;

    let payload = json!({
        "torrent_id": 1,
        "upload_factor": 200,
        "download_factor": 50
    });

    let req = test::TestRequest::put()
        .uri("/api/torrents/up-down-factors")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&payload)
        .to_request();

    let _response: serde_json::Value =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let torrent = pool.find_torrent(1).await.unwrap();
    assert_eq!(torrent.upload_factor, 200);
    assert_eq!(torrent.download_factor, 50);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_edit_torrent_up_down_factors_without_permission(pool: PgPool) {
    use serde_json::json;

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    let payload = json!({
        "torrent_id": 1,
        "upload_factor": 200,
        "download_factor": 50
    });

    let req = test::TestRequest::put()
        .uri("/api/torrents/up-down-factors")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .set_json(&payload)
        .to_request();

    common::call_and_read_body_json_with_status::<serde_json::Value, _>(
        &service,
        req,
        StatusCode::FORBIDDEN,
    )
    .await;
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_upload_torrent_blocked_by_release_date_cutoff(pool: PgPool) {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        error: String,
    }

    let pg_pool = pool.clone();

    // Set torrent_max_release_date_allowed to 1960-01-01
    // The test fixtures have content with release dates in 1962 and 1999
    sqlx::query("UPDATE arcadia_settings SET torrent_max_release_date_allowed = '1960-01-01'")
        .execute(&pg_pool)
        .await
        .unwrap();

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    use actix_multipart_rfc7578::client::multipart;

    let mut form = multipart::Form::default();
    form.add_text("release_name", "test release name");
    form.add_text("release_group", "TESTGROUP");
    form.add_text("description", "This is a test description");
    form.add_text("uploaded_as_anonymous", "true");
    form.add_text("mediainfo", "test mediainfo");
    form.add_text("languages", "English");
    form.add_text("container", "MKV");
    form.add_text("edition_group_id", "1"); // Edition group with 1962 release date
    form.add_text("duration", "3600");
    form.add_text("audio_codec", "flac");
    form.add_text("audio_bitrate", "1200");
    form.add_text("audio_channels", "5.1");
    form.add_text("audio_bitrate_sampling", "256");
    form.add_text("video_codec", "h264");
    form.add_text("features", "");
    form.add_text("subtitle_languages", "");
    form.add_text("video_resolution", "1080p");
    form.add_text("extras", "");
    form.add_text("bonus_points_snatch_cost", "0");

    let torrent_data = bytes::Bytes::from_static(include_bytes!(
        "data/debian-12.10.0-i386-netinst.iso.torrent"
    ));

    form.add_reader_file(
        "torrent_file",
        std::io::Cursor::new(torrent_data),
        "torrent_file.torrent",
    );

    let content_type = form.content_type();

    let payload = actix_web::body::to_bytes(multipart::Body::from(form))
        .await
        .unwrap();

    let req = test::TestRequest::post()
        .uri("/api/torrents")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Content-Type", content_type))
        .set_payload(payload)
        .to_request();

    let error: ErrorResponse =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::BAD_REQUEST).await;

    assert!(
        error
            .error
            .contains("content released after 1960-01-01 is not allowed"),
        "expected release date cutoff error, got: {:?}",
        error
    );
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_upload_torrent_bonus_points_cost_above_max(pool: PgPool) {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        error: String,
    }

    let pg_pool = pool.clone();

    sqlx::query("UPDATE arcadia_settings SET allow_uploader_set_torrent_bonus_points_cost = true, torrent_bonus_points_cost_min = 10, torrent_bonus_points_cost_max = 100")
        .execute(&pg_pool)
        .await
        .unwrap();

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    use actix_multipart_rfc7578::client::multipart;

    let mut form = multipart::Form::default();
    form.add_text("release_name", "test release name");
    form.add_text("release_group", "TESTGROUP");
    form.add_text("description", "This is a test description");
    form.add_text("uploaded_as_anonymous", "true");
    form.add_text("mediainfo", "test mediainfo");
    form.add_text("languages", "English");
    form.add_text("container", "MKV");
    form.add_text("edition_group_id", "1");
    form.add_text("duration", "3600");
    form.add_text("audio_codec", "flac");
    form.add_text("audio_bitrate", "1200");
    form.add_text("audio_channels", "5.1");
    form.add_text("audio_bitrate_sampling", "256");
    form.add_text("video_codec", "h264");
    form.add_text("features", "");
    form.add_text("subtitle_languages", "");
    form.add_text("video_resolution", "1080p");
    form.add_text("extras", "");
    form.add_text("bonus_points_snatch_cost", "150");

    let torrent_data = bytes::Bytes::from_static(include_bytes!(
        "data/debian-12.10.0-i386-netinst.iso.torrent"
    ));

    form.add_reader_file(
        "torrent_file",
        std::io::Cursor::new(torrent_data),
        "torrent_file.torrent",
    );

    let content_type = form.content_type();

    let payload = actix_web::body::to_bytes(multipart::Body::from(form))
        .await
        .unwrap();

    let req = test::TestRequest::post()
        .uri("/api/torrents")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Content-Type", content_type))
        .set_payload(payload)
        .to_request();

    let error: ErrorResponse =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::BAD_REQUEST).await;

    assert!(
        error
            .error
            .contains("bonus_points_snatch_cost must be at most 100"),
        "expected bonus points cost max error, got: {:?}",
        error
    );
}

#[sqlx::test(
    fixtures("with_test_users", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_upload_torrent_bonus_points_cost_below_min(pool: PgPool) {
    #[derive(Debug, Deserialize)]
    struct ErrorResponse {
        error: String,
    }

    let pg_pool = pool.clone();

    sqlx::query("UPDATE arcadia_settings SET allow_uploader_set_torrent_bonus_points_cost = true, torrent_bonus_points_cost_min = 10, torrent_bonus_points_cost_max = 100")
        .execute(&pg_pool)
        .await
        .unwrap();

    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), TestUser::Standard).await;

    use actix_multipart_rfc7578::client::multipart;

    let mut form = multipart::Form::default();
    form.add_text("release_name", "test release name");
    form.add_text("release_group", "TESTGROUP2");
    form.add_text("description", "This is a test description");
    form.add_text("uploaded_as_anonymous", "true");
    form.add_text("mediainfo", "test mediainfo");
    form.add_text("languages", "English");
    form.add_text("container", "MKV");
    form.add_text("edition_group_id", "1");
    form.add_text("duration", "3600");
    form.add_text("audio_codec", "flac");
    form.add_text("audio_bitrate", "1200");
    form.add_text("audio_channels", "5.1");
    form.add_text("audio_bitrate_sampling", "256");
    form.add_text("video_codec", "h264");
    form.add_text("features", "");
    form.add_text("subtitle_languages", "");
    form.add_text("video_resolution", "1080p");
    form.add_text("extras", "");
    form.add_text("bonus_points_snatch_cost", "5");

    let torrent_data = bytes::Bytes::from_static(include_bytes!(
        "data/debian-12.10.0-i386-netinst.iso.torrent"
    ));

    form.add_reader_file(
        "torrent_file",
        std::io::Cursor::new(torrent_data),
        "torrent_file.torrent",
    );

    let content_type = form.content_type();

    let payload = actix_web::body::to_bytes(multipart::Body::from(form))
        .await
        .unwrap();

    let req = test::TestRequest::post()
        .uri("/api/torrents")
        .insert_header(auth_header(&user.token))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Content-Type", content_type))
        .set_payload(payload)
        .to_request();

    let error: ErrorResponse =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::BAD_REQUEST).await;

    assert!(
        error
            .error
            .contains("bonus_points_snatch_cost must be at least 10"),
        "expected bonus points cost min error, got: {:?}",
        error
    );
}
