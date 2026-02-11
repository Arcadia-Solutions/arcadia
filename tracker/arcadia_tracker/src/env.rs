use envconfig::Envconfig;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Envconfig, Clone)]
pub struct Env {
    #[envconfig(from = "API_KEY")]
    pub api_key: String,
    #[envconfig(from = "ALLOWED_TORRENT_CLIENTS")]
    pub allowed_torrent_clients: AllowedTorrentClientSet,
    #[envconfig(from = "NUMWANT_DEFAULT")]
    pub numwant_default: usize,
    #[envconfig(from = "NUMWANT_MAX")]
    pub numwant_max: usize,
    #[envconfig(from = "ANNOUNCE_MIN")]
    pub announce_min: u32,
    #[envconfig(from = "ANNOUNCE_MIN_ENFORCED")]
    pub announce_min_enforced: u32,
    #[envconfig(from = "ANNOUNCE_MAX")]
    pub announce_max: u32,
    #[envconfig(from = "MAX_PEERS_PER_TORRENT_PER_USER")]
    pub max_peers_per_torrent_per_user: u8,
    #[envconfig(from = "FLUSH_INTERVAL_MILLISECONDS")]
    pub flush_interval_milliseconds: u64,
    #[envconfig(from = "PEER_EXPIRY_INTERVAL")]
    pub peer_expiry_interval: u64,
    #[envconfig(from = "REVERSE_PROXY_CLIENT_IP_HEADER_NAME")]
    pub reverse_proxy_client_ip_header_name: Option<String>,
    #[envconfig(from = "INACTIVE_PEER_TTL")]
    pub inactive_peer_ttl: u64,
    #[envconfig(from = "ACTIVE_PEER_TTL")]
    pub active_peer_ttl: u64,
    #[envconfig(from = "OTEL_SERVICE_NAME")]
    pub otel_service_name: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("env variable parse error '{0}'")]
    EnvVariableParseError(String),
}

#[derive(Debug, Clone)]
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
