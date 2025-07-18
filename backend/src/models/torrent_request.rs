use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::models::{torrent_request_vote::UserCreatedTorrentRequestVote, user::UserLite};

use super::torrent::{AudioBitrateSampling, AudioCodec, Features, Language, VideoCodec, VideoResolution};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequest {
    pub id: i64,
    pub title_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub filled_by_user_id: Option<i64>,
    pub filled_by_torrent_id: Option<i64>,
    #[schema(value_type = String, format = DateTime)]
    pub filled_at: Option<DateTime<Utc>>,
    pub edition_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<Language>,
    pub container: String,
    // ---- audio
    pub audio_codec: Option<AudioCodec>,
    pub audio_channels: Option<String>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>, // ---- video
    pub res_x: Option<i32>,
    pub res_y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedTorrentRequest {
    pub title_group_id: i64,
    pub edition_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<Language>,
    pub container: String,
    pub initial_vote: UserCreatedTorrentRequestVote,
    // ---- audio
    pub audio_codec: Option<AudioCodec>,
    pub audio_channels: Option<String>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>, // ---- video
    pub res_x: Option<i32>,
    pub res_y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestBounties {
    bonus_points: i64,
    upload: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestHierarchyLite {
    pub id: i64,
    pub title_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub created_by: UserLite,
    pub filled_by_user_id: Option<i64>,
    pub filled_by_torrent_id: Option<i64>,
    #[schema(value_type = String, format = DateTime)]
    pub filled_at: Option<DateTime<Utc>>,
    pub edition_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<Language>,
    pub container: String,
    pub bounties: TorrentRequestBounties,
    pub user_votes_amount: i32,
    // ---- audio
    pub audio_codec: Option<AudioCodec>,
    pub audio_channels: Option<String>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Option<VideoResolution>, // ---- video
    pub res_x: Option<i32>,
    pub res_y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestFill {
    pub torrent_request_id: i64,
    pub torrent_id: i64,
}
