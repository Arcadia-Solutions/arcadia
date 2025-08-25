use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::models::{
    artist::AffiliatedArtistHierarchy,
    edition_group::Source,
    series::SeriesLite,
    title_group::{TitleGroup, TitleGroupLite},
    torrent::AudioChannels,
    torrent_request_vote::{TorrentRequestVoteHierarchy, UserCreatedTorrentRequestVote},
    user::UserLite,
};

use super::torrent::{
    AudioBitrateSampling, AudioCodec, Features, Language, VideoCodec, VideoResolution,
};

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
    pub source: Vec<Source>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<Language>,
    pub container: Vec<String>,
    // ---- audio
    pub audio_codec: Vec<AudioCodec>,
    pub audio_channels: Vec<AudioChannels>,
    pub audio_bitrate_sampling: Vec<AudioBitrateSampling>,
    // ---- audio
    // ---- video
    pub video_codec: Vec<VideoCodec>,
    pub features: Vec<Features>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Vec<VideoResolution>, // ---- video
    pub video_resolution_other_x: Option<i32>,
    pub video_resolution_other_y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedTorrentRequest {
    pub title_group_id: i64,
    pub edition_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<Language>,
    pub container: Vec<String>,
    pub source: Vec<Source>,
    pub initial_vote: UserCreatedTorrentRequestVote,
    pub audio_codec: Vec<AudioCodec>,
    pub audio_channels: Vec<AudioChannels>,
    pub audio_bitrate_sampling: Vec<AudioBitrateSampling>,
    pub video_codec: Vec<VideoCodec>,
    pub features: Vec<Features>,
    pub subtitle_languages: Vec<Language>,
    pub video_resolution: Vec<VideoResolution>,
    pub video_resolution_other_x: Option<i32>,
    pub video_resolution_other_y: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestBounty {
    bonus_points: i64,
    upload: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestHierarchyLite {
    pub torrent_request: TorrentRequest,
    pub created_by: UserLite,
    pub user_votes_amount: i32,
    pub bounty: TorrentRequestBounty,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TorrentRequestWithTitleGroupLite {
    pub torrent_request: TorrentRequest,
    pub title_group: TitleGroupLite,
    pub bounty: TorrentRequestBounty,
    pub user_votes_amount: i32,
    pub affiliated_artists: Vec<AffiliatedArtistHierarchy>,
    pub series: SeriesLite,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestFill {
    pub torrent_request_id: i64,
    pub torrent_id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TorrentRequestAndAssociatedData {
    pub torrent_request: TorrentRequest,
    pub title_group: TitleGroup,
    pub affiliated_artists: Vec<AffiliatedArtistHierarchy>,
    pub series: SeriesLite,
    pub votes: Vec<TorrentRequestVoteHierarchy>,
}
