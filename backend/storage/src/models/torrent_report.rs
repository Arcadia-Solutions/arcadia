use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentReport {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub reported_at: DateTime<Local>,
    pub reported_by_id: i64,
    pub reported_torrent_id: i64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedTorrentReport {
    pub reported_torrent_id: i64,
    pub description: String,
}
