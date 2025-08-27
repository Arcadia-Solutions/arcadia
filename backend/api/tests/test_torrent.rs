use actix_web::{http::StatusCode, test};
use serde::Deserialize;
use sqlx::PgPool;

pub mod common;

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_valid_torrent(pool: PgPool) {
    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(token)
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
    fixtures("with_test_user", "with_test_title_group", "with_test_edition_group"),
    migrations = "../storage/migrations"
)]
async fn test_upload_torrent(pool: PgPool) {
    use actix_multipart_rfc7578::client::multipart;

    let mut form = multipart::Form::default();

    form.add_text("release_name", "test release name");
    form.add_text("release_group", "TESTGRoUP");
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

    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let req = test::TestRequest::post()
        .uri("/api/torrents")
        .insert_header(token)
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Content-Type", content_type))
        .set_payload(payload)
        .to_request();

    #[derive(Debug, Deserialize)]
    struct Torrent {
        edition_group_id: i64,
        created_by_id: i64,
    }

    let torrent = common::call_and_read_body_json_with_status::<Torrent, _>(
        &service,
        req,
        StatusCode::CREATED,
    )
    .await;

    assert_eq!(torrent.edition_group_id, 1);
    assert_eq!(torrent.created_by_id, 2);
}

#[derive(Debug, Deserialize)]
struct TitleGroupLite {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct TorrentSearchResults {
    title_groups: Option<Vec<TitleGroupLite>>,
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_find_torrents_by_external_link(pool: PgPool) {
    let link = "https://en.wikipedia.org/wiki/RollerCoaster_Tycoon";

    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let body = serde_json::json!({
        "title_group": { "name": link, "include_empty_groups": true },
        "torrent": {},
        "page": 1,
        "page_size": 50,
        "sort_by": "torrent_created_at",
        "order": "desc"
    });

    let req = test::TestRequest::post()
        .uri("/api/search/torrents/lite")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(token)
        .set_json(body)
        .to_request();

    let results: TorrentSearchResults =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let groups = results.title_groups.unwrap_or_default();
    assert!(
        groups
            .iter()
            .any(|g| g.id == 2 && g.name == "RollerCoaster Tycoon"),
        "expected results to include title_group id=2 (RollerCoaster Tycoon) when searching by external link"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_find_torrents_by_name(pool: PgPool) {
    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let body = serde_json::json!({
        "title_group": { "name": "Love Me Do", "include_empty_groups": true },
        "torrent": {},
        "page": 1,
        "page_size": 50,
        "sort_by": "torrent_created_at",
        "order": "desc"
    });

    let req = test::TestRequest::post()
        .uri("/api/search/torrents/lite")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(token)
        .set_json(body)
        .to_request();

    let results: TorrentSearchResults =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let groups = results.title_groups.unwrap_or_default();
    assert!(
        groups
            .iter()
            .any(|g| g.id == 1 && g.name == "Love Me Do / P.S. I Love You"),
        "expected results to include title_group id=1 (Love Me Do / P.S. I Love You) when searching by name"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../storage/migrations"
)]
async fn test_find_torrents_no_link_or_name_provided(pool: PgPool) {
    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let body = serde_json::json!({
        "title_group": { "name": "", "include_empty_groups": true },
        "torrent": {},
        "page": 1,
        "page_size": 50,
        "sort_by": "torrent_created_at",
        "order": "desc"
    });

    let req = test::TestRequest::post()
        .uri("/api/search/torrents/lite")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(token)
        .set_json(body)
        .to_request();

    let results: TorrentSearchResults =
        common::call_and_read_body_json_with_status(&service, req, StatusCode::OK).await;

    let groups = results.title_groups.unwrap_or_default();
    let ids: Vec<i64> = groups.iter().map(|g| g.id).collect();

    assert!(
        ids.contains(&1) && ids.contains(&2),
        "expected unfiltered results to include both title_group id=1 and id=2"
    );
}
