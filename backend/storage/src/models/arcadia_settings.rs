use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
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
    pub shop_upload_base_price_per_gb: i64,
    pub shop_upload_discount_tiers: serde_json::Value,
    pub shop_freeleech_token_base_price: i64,
    pub shop_freeleech_token_discount_tiers: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicArcadiaSettings {
    pub open_signups: bool,
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
    pub logo_subtitle: Option<String>,
}
