pub mod common;
pub mod mocks;

use std::sync::Arc;

use actix_web::test;
use arcadia_api::OpenSignups;
use arcadia_common::models::tracker::announce;
use arcadia_storage::connection_pool::ConnectionPool;
use mocks::mock_redis::MockRedisPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;

use crate::common::auth_header;

#[derive(Debug, Deserialize)]
struct WrappedError {
    #[serde(rename = "failure reason")]
    _failure_reason: String,
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_announce_unknown_passkey(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Enabled,
        1.0,
        1.0,
    )
    .await;

    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/33333333333333333333333333333333?",
            "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should fail because the passkey is invalid
    assert!(
        resp.status().is_client_error(),
        "status {} is not client error",
        resp.status()
    );

    // Any error is okay, as long as it has "failure reason" populated.
    common::read_body_bencode::<WrappedError, _>(resp)
        .await
        .expect("expected failure message");
}

#[sqlx::test(fixtures("with_test_user"), migrations = "../storage/migrations")]
async fn test_announce_unknown_torrent(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Enabled,
        1.0,
        1.0,
    )
    .await;

    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should fail because there is no torrent matching infohash.
    assert!(
        resp.status().is_client_error(),
        "status {} is not client error",
        resp.status()
    );

    // Any error is okay, as long as it has "failure reason" populated.
    common::read_body_bencode::<WrappedError, _>(resp)
        .await
        .expect("expected failure message");
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
async fn test_announce_known_torrent(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let service = common::create_test_app(
        pool,
        MockRedisPool::default(),
        OpenSignups::Enabled,
        1.0,
        1.0,
    )
    .await;
    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should succeed because there is both a matching user and info hash.
    assert!(
        resp.status().is_success(),
        "status {} is not success",
        resp.status()
    );

    let resp = common::read_body_bencode::<announce::AnnounceResponse, _>(resp)
        .await
        .expect("could not deserialize announce response");

    // There are no peers, so should be empty.
    assert!(resp.peers.is_empty());
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_user2",
        "with_test_peers"
    ),
    migrations = "../storage/migrations"
)]
async fn test_announce_known_torrent_with_peers(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), 1.0, 1.0).await;

    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=42&",
            "downloaded=43&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should succeed because there is both a matching user and info hash.
    assert!(
        resp.status().is_success(),
        "status {} is not success",
        resp.status()
    );

    let resp = common::read_body_bencode::<announce::AnnounceResponse, _>(resp)
        .await
        .expect("could not deserialize announce response");

    // Fixture sets up two non-self peers.
    assert!(resp.peers.len() == 2);

    for announce::Peer { ip, port } in &resp.peers {
        assert_ne!(
            (ip, port),
            (&std::net::Ipv4Addr::new(10, 10, 4, 88), &6968),
            "announce response contains self in peer list"
        );

        assert_ne!(
            (ip, port),
            (&std::net::Ipv4Addr::new(10, 10, 4, 91), &26),
            "peer by the same user is included in peer list"
        );
    }

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/me")
        .to_request();

    let body = common::call_and_read_body_json::<Value, _>(&service, req).await;

    assert_eq!(body["user"]["real_uploaded"].as_u64().unwrap(), 42);
    // should be 44 because users start with 1 byte downloaded at account creation
    assert_eq!(body["user"]["real_downloaded"].as_u64().unwrap(), 44);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_user2",
        "with_test_peers"
    ),
    migrations = "../storage/migrations"
)]
async fn test_announce_global_factor_manipulation(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), 2.0, 0.5).await;
    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=10&",
            "downloaded=10&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let _ = test::call_service(&service, req).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/me")
        .to_request();

    let body = common::call_and_read_body_json::<Value, _>(&service, req).await;

    assert_eq!(body["user"]["uploaded"].as_u64().unwrap(), 20);
    // should be 6 because users start with 1 byte downloaded at account creation
    assert_eq!(body["user"]["downloaded"].as_u64().unwrap(), 6);
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent_custom_up_down_factors",
        "with_test_user2",
        "with_test_peers"
    ),
    migrations = "../storage/migrations"
)]
async fn test_announce_torrent_specific_factor_manipulation(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), 1.0, 1.0).await;
    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=10&",
            "downloaded=10&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let _ = test::call_service(&service, req).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .uri("/api/users/me")
        .to_request();

    let body = common::call_and_read_body_json::<Value, _>(&service, req).await;

    assert_eq!(body["user"]["uploaded"].as_u64().unwrap(), 20);
    // should be 6 because users start with 1 byte downloaded at account creation
    assert_eq!(body["user"]["downloaded"].as_u64().unwrap(), 6);
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
async fn test_peers_after_announce(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let (service, user) =
        common::create_test_app_and_login(pool, MockRedisPool::default(), 1.0, 1.0).await;

    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=100&",
            "downloaded=100&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should succeed because there is both a matching user and info hash.
    assert!(
        resp.status().is_success(),
        "status {} is not success",
        resp.status()
    );

    let _ = common::read_body_bencode::<announce::AnnounceResponse, _>(resp)
        .await
        .expect("could not deserialize announce response");

    let req = test::TestRequest::get()
        .uri("/api/users/me")
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(auth_header(&user.token))
        .to_request();

    #[derive(Debug, PartialEq, Deserialize)]
    struct Peer {
        pub ip: String,
        pub port: i16,
        pub real_uploaded: i64,
        pub real_downloaded: i64,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Profile {
        pub peers: Vec<Peer>,
    }

    let resp = common::call_and_read_body_json::<Profile, _>(&service, req).await;

    assert_eq!(
        resp.peers,
        vec![Peer {
            ip: String::from("10.10.4.88/32"),
            port: 6968,
            real_uploaded: 100,
            real_downloaded: 100,
        }]
    );
}
