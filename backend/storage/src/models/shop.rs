use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "shop_item", rename_all = "snake_case")]
pub enum ShopItem {
    Promotion,
    Upload,
    FreeleechTokens,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ShopPurchase {
    pub id: i64,
    pub user_id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub purchased_at: DateTime<Utc>,
    pub item_type: ShopItem,
    pub bonus_points_spent: i64,
    pub quantity: i64,
    pub extra_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BuyUploadRequest {
    pub bytes: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BuyFreeleechTokensRequest {
    pub quantity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UploadDiscountTier {
    pub threshold_gb: i64,
    pub discount_percent: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FreeleechTokenDiscountTier {
    pub threshold: i32,
    pub discount_percent: i16,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PromotionPricing {
    pub next_class_name: String,
    pub cost: i64,
    pub requirements_met: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ShopPricing {
    pub upload_base_price_per_gb: i64,
    pub upload_discount_tiers: Vec<UploadDiscountTier>,
    pub freeleech_token_base_price: i64,
    pub freeleech_token_discount_tiers: Vec<FreeleechTokenDiscountTier>,
    pub promotion: Option<PromotionPricing>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadPriceCalculation {
    pub bytes: i64,
    pub base_price: i64,
    pub discount_percent: i16,
    pub final_price: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FreeleechTokensPriceCalculation {
    pub quantity: i32,
    pub base_price: i64,
    pub discount_percent: i16,
    pub final_price: i64,
}
