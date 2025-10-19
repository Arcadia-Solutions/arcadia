use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::models::user::UserLiteAvatar;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestComment {
    pub id: i64,
    pub torrent_request_id: i64,
    pub created_by_id: i32,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedTorrentRequestComment {
    pub torrent_request_id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestCommentHierarchy {
    pub id: i64,
    pub torrent_request_id: i64,
    pub created_by_id: i32,
    pub created_by: UserLiteAvatar,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}
