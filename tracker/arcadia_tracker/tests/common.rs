use actix_http::Request;
use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    test, web, App, Error,
};
use arcadia_shared::tracker::models::{
    env::ArcadiaSettingsForTracker, infohash_2_id, passkey_2_id, torrent, user,
};
use arcadia_tracker::{
    env::{AllowedTorrentClientSet, Env},
    routes::init,
    Tracker,
};
use parking_lot::{Mutex, RwLock};
use serde::de::DeserializeOwned;
use sqlx::PgPool;
use std::sync::OnceLock;

pub async fn create_test_app(
    pool: PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    // Create a default env for testing
    let env = Env {
        api_key: "amazing_api_key".to_owned(),
        allowed_torrent_clients: AllowedTorrentClientSet {
            clients: vec![b"lt0F01-".to_vec(), b"qB".to_vec(), b"UTorrent".to_vec()]
                .into_iter()
                .collect(),
        },
        numwant_default: 50,
        numwant_max: 200,
        announce_min: 1800,
        announce_min_enforced: 0, // Disable rate limiting for tests
        announce_max: 7200,
        max_peers_per_torrent_per_user: 10,
        flush_interval_milliseconds: 60000,
        peer_expiry_interval: 600,
        reverse_proxy_client_ip_header_name: None,
        inactive_peer_ttl: 300,
        active_peer_ttl: 3600,
        otel_service_name: None,
    };

    // Load data from test database
    let settings = ArcadiaSettingsForTracker::from_database(&pool).await;
    let users = user::Map::from_database(&pool).await;
    let passkey2id = passkey_2_id::Map::from_database(&pool).await;
    let infohash2id = infohash_2_id::Map::from_database(&pool).await;
    let torrents = torrent::Map::from_database(&pool).await;

    let tracker = Tracker {
        env,
        pool,
        settings: RwLock::new(settings),
        metrics: OnceLock::new(),
        users: RwLock::new(users),
        passkey2id: RwLock::new(passkey2id),
        infohash2id: RwLock::new(infohash2id),
        torrents: Mutex::new(torrents),
        user_updates: Mutex::new(Default::default()),
        torrent_updates: Mutex::new(Default::default()),
        peer_updates: Mutex::new(Default::default()),
    };

    test::init_service(App::new().app_data(web::Data::new(tracker)).configure(init)).await
}

pub async fn read_body_bencode<T: DeserializeOwned, B: MessageBody>(
    resp: ServiceResponse<B>,
) -> Result<T, serde_bencode::Error> {
    let body = test::read_body(resp).await;
    serde_bencode::from_bytes(&body)
}
