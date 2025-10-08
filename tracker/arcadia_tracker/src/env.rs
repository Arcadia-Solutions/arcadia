use envconfig::Envconfig;
use std::{collections::HashSet, str::FromStr};

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(from = "ANNOUNCE_INTERVAL")]
    pub announce_interval: u32,
    #[envconfig(from = "ANNOUNCE_INTERVAL_GRACE_PERIOD")]
    pub announce_interval_grace_period: u32,
    #[envconfig(from = "ALLOWED_TORRENT_CLIENTS")]
    pub allowed_torrent_clients: AllowedTorrentClientSet,
    // #[envconfig(from = "GLOBAL_UPLOAD_FACTOR")]
    // pub global_upload_factor: f64,
    // #[envconfig(from = "GLOBAL_DOWNLOAD_FACTOR")]
    // pub global_download_factor: f64,
    #[envconfig(from = "TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS")]
    pub interval_update_torrent_seeders_leechers: String,
    #[envconfig(from = "TASK_INTERVAL_REMOVE_INACTIVE_PEERS")]
    pub interval_remove_inactive_peers: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("env variable parse error '{0}'")]
    EnvVariableParseError(String),
}

#[derive(Clone)]
pub struct AllowedTorrentClientSet {
    pub clients: HashSet<Vec<u8>>,
}

impl FromStr for AllowedTorrentClientSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clients = s
            .split(',')
            .map(|s| s.trim().as_bytes().to_vec())
            .collect::<HashSet<Vec<u8>>>();

        Ok(Self { clients })
    }
}
