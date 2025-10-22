use arcadia_shared::tracker::models::{
    user_update::{self, UserUpdate},
    Queue,
};
use tokio::sync::{Mutex, RwLock};

use crate::env::Env;
use std::{io::Write, ops::Deref};

pub mod announce;
pub mod api_doc;
pub mod env;
pub mod middleware;
pub mod routes;
pub mod scheduler;
pub mod services;

#[derive(Debug)]
pub struct Tracker {
    env: Env,

    pub users: RwLock<arcadia_shared::tracker::models::user::Map>,
    pub passkey2id: RwLock<arcadia_shared::tracker::models::passkey_2_id::Map>,
    pub infohash2id: RwLock<arcadia_shared::tracker::models::infohash_2_id::Map>,
    pub torrents: Mutex<arcadia_shared::tracker::models::torrent::Map>,
    pub user_updates: Mutex<Queue<user_update::Index, UserUpdate>>,
}

impl Deref for Tracker {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Tracker {
    pub async fn new(mut env: Env) -> Self {
        log::info!("[Setup] Getting shared env...");
        std::io::stdout().flush().unwrap();
        let shared_env = arcadia_shared::tracker::models::env::Env::from_backend().await;
        env.global_download_factor = shared_env.global_download_factor;
        env.global_upload_factor = shared_env.global_upload_factor;
        log::info!("[Setup] Got shared env");
        println!("{:?}", env);

        log::info!("[Setup] Getting users...");
        std::io::stdout().flush().unwrap();
        let users = arcadia_shared::tracker::models::user::Map::from_backend().await;
        log::info!("[Setup] Got {:?} users", users.len());

        log::info!("[Setup] Getting passkey2id...");
        std::io::stdout().flush().unwrap();
        let passkey2id = arcadia_shared::tracker::models::passkey_2_id::Map::from_backend().await;
        log::info!("[Setup] Got {:?} passkey2ids", passkey2id.len());

        log::info!("[Setup] Getting infohash2id...");
        std::io::stdout().flush().unwrap();
        let infohash2id = arcadia_shared::tracker::models::infohash_2_id::Map::from_backend().await;
        log::info!("[Setup] Got {:?} infohash2ids", infohash2id.len());

        log::info!("[Setup] Getting torrents...");
        std::io::stdout().flush().unwrap();
        let torrents = arcadia_shared::tracker::models::torrent::Map::from_backend().await;
        log::info!("[Setup] Got {:?} torrents", torrents.len());

        Self {
            env,
            users: RwLock::new(users),
            passkey2id: RwLock::new(passkey2id),
            infohash2id: RwLock::new(infohash2id),
            torrents: Mutex::new(torrents),
            user_updates: Mutex::new(Queue::<user_update::Index, UserUpdate>::default()),
        }
    }
}
