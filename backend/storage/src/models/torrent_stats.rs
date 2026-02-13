use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StatsInterval {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TorrentStatsGroupBy {
    None,
    VideoResolution,
    VideoCodec,
    AudioCodec,
    AudioChannels,
    AudioBitrateSampling,
    Container,
    Source,
    ContentType,
    Category,
    Platform,
    OriginalLanguage,
    CountryFrom,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct TorrentStatsQuery {
    #[param(value_type = String, format = "date")]
    pub from: chrono::NaiveDate,
    #[param(value_type = String, format = "date")]
    pub to: chrono::NaiveDate,
    pub interval: StatsInterval,
    pub group_by: TorrentStatsGroupBy,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentStatsDataPoint {
    #[schema(value_type = String, format = DateTime)]
    pub period: NaiveDateTime,
    pub count: i64,
    pub total_size: i64,
    pub attribute_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TorrentStatsResponse {
    pub unique_uploaders: i64,
    pub data: Vec<TorrentStatsDataPoint>,
}
