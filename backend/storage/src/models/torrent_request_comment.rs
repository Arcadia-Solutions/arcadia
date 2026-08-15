use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

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

/// Query of the paginated list of every torrent request comment written by a user.
#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct UserTorrentRequestCommentSearchQuery {
    pub created_by_id: i32,
    pub page: u32,
    pub page_size: u32,
}

/// A torrent request comment displayed outside of its torrent request: it carries the title
/// group the request was made for.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TorrentRequestCommentWithLocation {
    #[serde(flatten)]
    pub comment: TorrentRequestCommentHierarchy,
    pub title_group_id: i32,
    pub title_group_name: String,
}
