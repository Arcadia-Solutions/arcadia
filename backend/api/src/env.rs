use crate::OpenSignups;
use envconfig::Envconfig;
use reqwest::Url;
use std::{collections::HashSet, str::FromStr};

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(nested)]
    pub actix: ActixConfig,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
    #[envconfig(from = "ARCADIA_OPEN_SIGNUPS")]
    pub open_signups: OpenSignups,
    #[envconfig(from = "ARCADIA_FRONTEND_URL")]
    pub frontend_url: Url,
    #[envconfig(nested)]
    pub tracker: TrackerConfig,
    #[envconfig(nested)]
    pub smtp: SmtpConfig,
    #[envconfig(nested)]
    pub redis: RedisConfig,
    #[envconfig(from = "TMDB_API_KEY")]
    pub tmdb_api_key: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("env variable parse error '{0}'")]
    EnvVariableParseError(String),
}

#[derive(Envconfig, Clone)]
pub struct ActixConfig {
    #[envconfig(from = "ACTIX_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "ACTIX_PORT", default = "8080")]
    pub port: u16,
}

#[derive(Envconfig, Clone)]
pub struct RedisConfig {
    #[envconfig(from = "REDIS_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "REDIS_PASSWORD")]
    pub password: String,
    #[envconfig(from = "REDIS_PORT", default = "6379")]
    pub port: u16,
}

#[derive(Envconfig, Clone)]
pub struct TrackerConfig {
    #[envconfig(from = "ARCADIA_TRACKER_NAME")]
    pub name: String,
    #[envconfig(from = "ARCADIA_TRACKER_URL")]
    pub url: Url,
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL")]
    pub announce_interval: u32,
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD")]
    pub announce_interval_grace_period: u32,
    #[envconfig(from = "ARCADIA_ALLOWED_TORRENT_CLIENTS")]
    pub allowed_torrent_clients: AllowedTorrentClientSet,
    #[envconfig(from = "ARCADIA_GLOBAL_UPLOAD_FACTOR")]
    pub global_upload_factor: f64,
    #[envconfig(from = "ARCADIA_GLOBAL_DOWNLOAD_FACTOR")]
    pub global_download_factor: f64,
    #[envconfig(from = "TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS")]
    pub interval_update_torrent_seeders_leechers: String,
    #[envconfig(from = "TASK_INTERVAL_REMOVE_INACTIVE_PEERS")]
    pub interval_remove_inactive_peers: String,
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

#[derive(Envconfig, Clone)]
pub struct SmtpConfig {
    #[envconfig(from = "SMTP_HOST")]
    pub host: Option<String>,
    #[envconfig(from = "SMTP_PORT")]
    pub port: Option<u16>,
    #[envconfig(from = "SMTP_USERNAME")]
    pub username: Option<String>,
    #[envconfig(from = "SMTP_PASSWORD")]
    pub password: Option<String>,
    #[envconfig(from = "SMTP_FROM_EMAIL")]
    pub from_email: Option<String>,
    #[envconfig(from = "SMTP_FROM_NAME")]
    pub from_name: Option<String>,
}
