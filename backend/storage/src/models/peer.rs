use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;

use super::user::UserLite;

#[derive(Debug, Deserialize, Serialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "peer_status_enum", rename_all = "lowercase")]
pub enum PeerStatus {
    Seeding,
    Leeching,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Peer {
    #[schema(value_type = String, format = "0.0.0.0")]
    pub ip: IpNetwork,
    pub port: i32,
    #[schema(value_type = String, format = DateTime)]
    pub first_seen_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen_at: DateTime<Local>,
    pub real_uploaded: i64,
    pub real_downloaded: i64,
    pub agent: Option<String>,
    pub status: PeerStatus,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicPeer {
    pub user: UserLite,
    #[schema(value_type = Option<String>, format = "0.0.0.0")]
    pub ip: Option<IpNetwork>,
    pub port: Option<i32>,
    pub uploaded: i64,
    pub downloaded: i64,
    pub left: i64,
    pub seeder: bool,
    pub agent: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TorrentClient {
    #[schema(value_type = String, format = "0.0.0.0")]
    pub ip: IpNetwork,
    pub port: i32,
    #[schema(value_type = String, format = DateTime)]
    pub first_seen_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen_at: DateTime<Utc>,
    pub real_uploaded: i64,
    pub real_downloaded: i64,
    pub agent: String,
}
