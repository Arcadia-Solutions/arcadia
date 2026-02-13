pub use arcadia_shared::tracker::models::env::SnatchedTorrentBonusPointsTransferredTo;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Json;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "displayed_top_bar_stats_enum", rename_all = "snake_case")]
pub enum DisplayedTopBarStats {
    Uploaded,
    Downloaded,
    Ratio,
    Torrents,
    ForumPosts,
    Seeding,
    Leeching,
    SeedingSize,
    AverageSeedingTime,
    BonusPoints,
    FreeleechTokens,
    CurrentStreak,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ArcadiaSettings {
    pub user_class_name_on_signup: String,
    pub default_css_sheet_name: String,
    pub open_signups: bool,
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
    pub logo_subtitle: Option<String>,
    pub approved_image_hosts: Vec<String>,
    pub upload_page_top_text: Option<String>,
    pub automated_message_on_signup: Option<String>,
    pub automated_message_on_signup_sender_id: Option<i32>,
    pub automated_message_on_signup_locked: Option<bool>,
    pub automated_message_on_signup_conversation_name: Option<String>,
    pub bonus_points_given_on_upload: i64,
    pub allow_uploader_set_torrent_bonus_points_cost: bool,
    pub default_torrent_bonus_points_cost: i64,
    pub torrent_bonus_points_cost_min: i64,
    pub torrent_bonus_points_cost_max: i64,
    pub shop_upload_base_price_per_gb: i64,
    pub shop_upload_discount_tiers: serde_json::Value,
    pub shop_freeleech_token_base_price: i64,
    pub shop_freeleech_token_discount_tiers: serde_json::Value,
    pub bonus_points_alias: String,
    pub bonus_points_decimal_places: i16,
    #[schema(value_type = Option<String>)]
    pub torrent_max_release_date_allowed: Option<NaiveDate>,
    pub snatched_torrent_bonus_points_transferred_to:
        Option<SnatchedTorrentBonusPointsTransferredTo>,
    pub displayed_top_bar_stats: Vec<DisplayedTopBarStats>,
    #[schema(value_type = Vec<BonusPointsEndpoint>)]
    pub bonus_points_per_endpoint: Json<Vec<BonusPointsEndpoint>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct BonusPointsEndpoint {
    pub method: HttpMethod,
    pub path_prefix: String,
    pub probability: i16,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicArcadiaSettings {
    pub open_signups: bool,
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
    pub logo_subtitle: Option<String>,
    pub bonus_points_alias: String,
    pub bonus_points_decimal_places: i16,
    pub displayed_top_bar_stats: Vec<DisplayedTopBarStats>,
}
