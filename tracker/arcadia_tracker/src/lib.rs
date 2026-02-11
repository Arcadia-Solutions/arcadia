use arcadia_shared::tracker::models::{
    env::ArcadiaSettingsForTracker,
    peer_update::{self, PeerUpdate},
    torrent_update::{self, TorrentUpdate},
    user_update::{self, UserUpdate},
    Queue,
};
use parking_lot::{Mutex, RwLock};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::env::Env;
use std::{io::Write, ops::Deref, sync::OnceLock, time::Duration};

pub mod announce;
pub mod api_doc;
pub mod env;
pub mod handlers;
pub mod metrics;
pub mod middleware;
pub mod routes;
pub mod scheduler;
pub mod services;

#[derive(Debug)]
pub struct Tracker {
    pub env: Env,

    pub pool: PgPool,

    pub settings: RwLock<ArcadiaSettingsForTracker>,

    pub metrics: OnceLock<metrics::Instruments>,

    pub users: RwLock<arcadia_shared::tracker::models::user::Map>,
    pub passkey2id: RwLock<arcadia_shared::tracker::models::passkey_2_id::Map>,
    pub infohash2id: RwLock<arcadia_shared::tracker::models::infohash_2_id::Map>,
    pub torrents: Mutex<arcadia_shared::tracker::models::torrent::Map>,
    pub user_updates: Mutex<Queue<user_update::Index, UserUpdate>>,
    pub torrent_updates: Mutex<Queue<torrent_update::Index, TorrentUpdate>>,
    pub peer_updates: Mutex<Queue<peer_update::Index, PeerUpdate>>,
}

impl Deref for Tracker {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Tracker {
    pub async fn new(env: Env) -> Self {
        println!("{:?}", env);

        print!("Connecting to database... ");
        std::io::stdout().flush().unwrap();
        let pool = connect_to_database().await;
        println!("[Finished]");

        log::info!("[Setup] Getting shared settings from database...");
        std::io::stdout().flush().unwrap();
        let settings = ArcadiaSettingsForTracker::from_database(&pool).await;
        log::info!("[Setup] Got settings: {:?}", settings);

        log::info!("[Setup] Getting users...");
        std::io::stdout().flush().unwrap();
        let users = arcadia_shared::tracker::models::user::Map::from_database(&pool).await;
        log::info!("[Setup] Got {:?} users", users.len());

        log::info!("[Setup] Getting passkey2id...");
        std::io::stdout().flush().unwrap();
        let passkey2id =
            arcadia_shared::tracker::models::passkey_2_id::Map::from_database(&pool).await;
        log::info!("[Setup] Got {:?} passkey2ids", passkey2id.len());

        log::info!("[Setup] Getting infohash2id...");
        std::io::stdout().flush().unwrap();
        let infohash2id =
            arcadia_shared::tracker::models::infohash_2_id::Map::from_database(&pool).await;
        log::info!("[Setup] Got {:?} infohash2ids", infohash2id.len());

        log::info!("[Setup] Getting torrents...");
        std::io::stdout().flush().unwrap();
        let torrents = arcadia_shared::tracker::models::torrent::Map::from_database(&pool).await;
        log::info!("[Setup] Got {:?} torrents", torrents.len());

        Self {
            env,
            pool,
            settings: RwLock::new(settings),
            metrics: OnceLock::new(),
            users: RwLock::new(users),
            passkey2id: RwLock::new(passkey2id),
            infohash2id: RwLock::new(infohash2id),
            torrents: Mutex::new(torrents),
            user_updates: Mutex::new(Queue::<user_update::Index, UserUpdate>::default()),
            torrent_updates: Mutex::new(Queue::<torrent_update::Index, TorrentUpdate>::default()),
            peer_updates: Mutex::new(Queue::<peer_update::Index, PeerUpdate>::default()),
        }
    }
}

async fn connect_to_database() -> sqlx::Pool<sqlx::Postgres> {
    // Get pool of database connections.
    PgPoolOptions::new()
        .min_connections(0)
        .max_connections(60)
        .max_lifetime(Duration::from_secs(30 * 60))
        .idle_timeout(Duration::from_secs(10 * 60))
        .acquire_timeout(Duration::from_secs(30))
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file. Aborting."))
        .await
        .expect("Could not connect to the database using the DATABASE_URL value in .env file. Aborting.")
}
