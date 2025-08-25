use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentActivity {
    pub id: i64,
    pub torrent_id: i64,
    pub user_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub snatched_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub first_seen_seeding_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen_seeding_at: DateTime<Local>,
    pub total_seed_time: i64, // in seconds
}
