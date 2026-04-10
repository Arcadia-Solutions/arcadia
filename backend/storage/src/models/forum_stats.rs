use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

use super::torrent_stats::StatsInterval;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ForumStatsMetric {
    Threads,
    Posts,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ForumStatsGroupBy {
    None,
    Category,
    SubCategory,
    Thread,
    User,
    UserClass,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ForumStatsQuery {
    #[param(value_type = String, format = "date")]
    pub from: chrono::NaiveDate,
    #[param(value_type = String, format = "date")]
    pub to: chrono::NaiveDate,
    pub interval: StatsInterval,
    pub group_by: ForumStatsGroupBy,
    pub metric: ForumStatsMetric,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ForumStatsDataPoint {
    #[schema(value_type = String, format = DateTime)]
    pub period: NaiveDateTime,
    pub count: i64,
    pub total_content_length: i64,
    pub attribute_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ForumStatsResponse {
    pub unique_posters: i64,
    pub unique_thread_creators: i64,
    pub total_threads_created: i64,
    pub total_posts_created: i64,
    pub total_content_length: i64,
    pub average_posts_per_thread: f64,
    pub average_post_length: f64,
    pub data: Vec<ForumStatsDataPoint>,
}
