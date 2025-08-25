use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Peer {
    pub id: i64,
    pub user_id: i64,
    pub torrent_id: i64,
    pub peer_id: [u8; 20],
    pub ip: Option<std::net::Ipv4Addr>,
    pub port: u16,
    pub uploaded: i64,
    pub downloaded: i64,
    pub first_seen_at: DateTime<Local>,
    pub last_seen_at: DateTime<Local>,
}
