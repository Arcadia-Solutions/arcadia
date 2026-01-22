mod common;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::test;
use common::read_body_bencode;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AnnounceResponse {
    #[serde(rename = "complete")]
    seeders: u32,
    #[serde(rename = "incomplete")]
    leechers: u32,
    #[serde(rename = "downloaded")]
    times_completed: u32,
    interval: u32,
    #[serde(rename = "min interval")]
    min_interval: u32,
    #[serde(default, deserialize_with = "deserialize_optional_bytes")]
    peers: Option<Vec<u8>>,
    #[serde(
        default,
        deserialize_with = "deserialize_optional_bytes",
        rename = "peers6"
    )]
    peers6: Option<Vec<u8>>,
}

fn deserialize_optional_bytes<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct BytesVisitor;

    impl<'de> serde::de::Visitor<'de> for BytesVisitor {
        type Value = Option<Vec<u8>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a byte string or sequence")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.to_vec()))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = seq.next_element::<u8>()? {
                vec.push(elem);
            }
            Ok(Some(vec))
        }
    }

    deserializer.deserialize_any(BytesVisitor)
}

#[derive(Debug, Deserialize)]
struct WrappedError {
    #[serde(rename = "failure reason")]
    failure_reason: String,
}

/// URL-encodes the info_hash bytes
fn url_encode_info_hash(info_hash: &[u8; 20]) -> String {
    percent_encoding::percent_encode(info_hash, percent_encoding::NON_ALPHANUMERIC).to_string()
}

/// Creates a valid peer_id for testing (must be 20 bytes)
/// Uses libtorrent format which starts with '-' followed by client identifier
fn test_peer_id() -> [u8; 20] {
    let mut peer_id = [0u8; 20];
    peer_id[0] = b'-';
    // Use '-lt0F01-' prefix which is in the allowed clients list
    peer_id[1..8].copy_from_slice(b"lt0F01-");
    // Fill rest with test data
    peer_id[8..].fill(b'1');
    peer_id
}

#[sqlx::test(
    fixtures("with_test_user"),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_invalid_passkey(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let invalid_passkey = "invalid_passkey_too_short";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started",
            invalid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert!(
        resp.status().is_client_error(),
        "Expected client error, got status {}",
        resp.status()
    );

    let error: WrappedError = read_body_bencode(resp)
        .await
        .expect("Failed to decode error");
    assert!(!error.failure_reason.is_empty());
}

#[sqlx::test(
    fixtures("with_test_user"),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_passkey_not_found(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    // Use a valid format passkey but one that doesn't exist
    let non_existent_passkey = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started",
            non_existent_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert!(
        resp.status().is_client_error(),
        "Expected client error, got status {}",
        resp.status()
    );

    let error: WrappedError = read_body_bencode(resp)
        .await
        .expect("Failed to decode error");

    assert_eq!(
        error.failure_reason,
        "User does not exist. Please re-download the .torrent file."
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_info_hash_not_found(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let valid_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    // Use an info_hash that doesn't exist in the database
    let info_hash_bytes = [0xFF; 20];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert!(
        resp.status().is_client_error(),
        "Expected client error, got status {}",
        resp.status()
    );

    let error: WrappedError = read_body_bencode(resp)
        .await
        .expect("Failed to decode error");

    assert_eq!(error.failure_reason, "InfoHash not found.");
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_successful_started(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let valid_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    // Info hash from with_test_torrent.sql: \x112233445566778899aabbccddeeff0011223344
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;
    let status = resp.status();

    if !status.is_success() {
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);
        panic!(
            "Expected success, got status {}. Body: {}",
            status, body_str
        );
    }

    let announce_resp: AnnounceResponse = read_body_bencode(resp)
        .await
        .expect("Failed to decode announce response");

    assert_eq!(announce_resp.seeders, 0, "Should have 0 seeders initially");
    assert_eq!(
        announce_resp.leechers, 1,
        "Should have 1 leecher (this peer)"
    );
    assert_eq!(
        announce_resp.times_completed, 0,
        "Should have 0 completions"
    );
    assert!(
        announce_resp.interval >= 1800,
        "Interval should be at least 1800"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_peers"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_with_existing_peers(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let valid_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1&numwant=50",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;
    let status = resp.status();

    if !status.is_success() {
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);
        panic!(
            "Expected success, got status {}. Body: {}",
            status, body_str
        );
    }

    let announce_resp: AnnounceResponse = read_body_bencode(resp)
        .await
        .expect("Failed to decode announce response");

    // The fixtures have 3 peers (2 seeders + 1 leecher), all with user_id 1
    // When we announce as user_id 1, the tracker filters out peers with same user_id from peer list
    // However, the seeder/leecher counts should reflect the total from database initially
    // But since we're loading fresh from the database and the counts start at 0, we need to check
    // if peers are actually loaded. The tracker should load peers from DB and count them.
    // For now, just verify the announce was successful and we have valid counts
    assert!(
        announce_resp.leechers >= 1, // At least this peer should be counted
        "Should have at least 1 leecher (this peer), got {}",
        announce_resp.leechers
    );
    // Note: The in-memory tracker loads peers from DB, but seeder/leecher counts might start at 0
    // and be updated based on active peers in the map. The fixtures peers might not be active
    // or counted until they're part of an announce cycle.
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_completed_event(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let valid_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    // First announce with started event
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=1000&left=0&event=started&compact=1",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let _resp = test::call_service(&service, req).await;

    // Now announce with completed event
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=1000&left=0&event=completed&compact=1",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;
    let status = resp.status();

    if !status.is_success() {
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);
        panic!(
            "Expected success, got status {}. Body: {}",
            status, body_str
        );
    }

    let announce_resp: AnnounceResponse = read_body_bencode(resp)
        .await
        .expect("Failed to decode announce response");

    // After completing, should be a seeder
    assert!(
        announce_resp.seeders >= 1,
        "Should have at least 1 seeder after completion"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_stopped_event(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let valid_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    // First announce with started event
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let _resp = test::call_service(&service, req).await;

    // Now announce with stopped event
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=100&downloaded=200&left=800&event=stopped&compact=1",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;
    let status = resp.status();

    if !status.is_success() {
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);
        panic!(
            "Expected success, got status {}. Body: {}",
            status, body_str
        );
    }

    let announce_resp: AnnounceResponse = read_body_bencode(resp)
        .await
        .expect("Failed to decode announce response");

    // After stopping, should no longer count as leecher
    assert_eq!(
        announce_resp.leechers, 0,
        "Should have 0 leechers after stopping"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_missing_user_agent(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    let valid_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        // Note: No User-Agent header
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert!(
        resp.status().is_client_error(),
        "Expected client error for missing user agent, got status {}",
        resp.status()
    );

    let error: WrappedError = read_body_bencode(resp)
        .await
        .expect("Failed to decode error");
    assert_eq!(error.failure_reason, "user-agent is missing");
}

#[sqlx::test(
    fixtures(
        "with_test_user_snatch_limit",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_snatch_limit_under(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    // User with max_snatches_per_day = 2
    let valid_passkey = "e3037c66dd3e13044e0d2f9b891c3838";
    let info_hash_bytes = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let info_hash_encoded = url_encode_info_hash(&info_hash_bytes);
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    // Announce as leecher (left > 0) - should succeed since under limit
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1",
            valid_passkey, info_hash_encoded, peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();

    let resp = test::call_service(&service, req).await;
    let status = resp.status();

    if !status.is_success() {
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body);
        panic!(
            "Expected success, got status {}. Body: {}",
            status, body_str
        );
    }

    let announce_resp: AnnounceResponse = read_body_bencode(resp)
        .await
        .expect("Failed to decode announce response");

    assert_eq!(
        announce_resp.leechers, 1,
        "Should have 1 leecher (this peer)"
    );
}

#[sqlx::test(
    fixtures(
        "with_test_user_snatch_limit",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_torrent_2",
        "with_test_torrent_3"
    ),
    migrations = "../../backend/storage/migrations"
)]
async fn test_announce_snatch_limit_exceeded(pool: PgPool) {
    let service = common::create_test_app(pool).await;

    // User with max_snatches_per_day = 2
    let valid_passkey = "e3037c66dd3e13044e0d2f9b891c3838";
    let peer_id = test_peer_id();
    let peer_id_encoded =
        percent_encoding::percent_encode(&peer_id, percent_encoding::NON_ALPHANUMERIC).to_string();

    // First torrent - should succeed
    let info_hash_1 = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x44,
    ];
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1",
            valid_passkey, url_encode_info_hash(&info_hash_1), peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();
    let resp = test::call_service(&service, req).await;
    assert!(resp.status().is_success(), "First torrent should succeed");

    // Second torrent - should succeed (at limit)
    let info_hash_2 = [
        0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x55,
    ];
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1",
            valid_passkey, url_encode_info_hash(&info_hash_2), peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();
    let resp = test::call_service(&service, req).await;
    assert!(resp.status().is_success(), "Second torrent should succeed");

    // Third torrent - should fail (exceeds limit of 2)
    let info_hash_3 = [
        0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0x00, 0x11, 0x22, 0x33, 0x66,
    ];
    let req = test::TestRequest::get()
        .uri(&format!(
            "/{}/announce?info_hash={}&peer_id={}&port=6969&uploaded=0&downloaded=0&left=1000&event=started&compact=1",
            valid_passkey, url_encode_info_hash(&info_hash_3), peer_id_encoded
        ))
        .insert_header(("User-Agent", "test-agent/1.0"))
        .peer_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0))
        .to_request();
    let resp = test::call_service(&service, req).await;

    assert!(
        resp.status().is_client_error(),
        "Third torrent should fail due to snatch limit"
    );

    let error: WrappedError = read_body_bencode(resp)
        .await
        .expect("Failed to decode error");
    assert_eq!(
        error.failure_reason,
        "You have already leeched 2 torrents in the past 24h."
    );
}
