use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Donation {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub amount: f64,
    pub currency: String,
    pub donor_name: Option<String>,
    pub user_id: Option<i32>,
    pub note: String,
    pub created_by_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedDonation {
    pub amount: f64,
    pub currency: String,
    pub donor_name: Option<String>,
    pub user_id: Option<i32>,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedDonation {
    pub amount: f64,
    pub currency: String,
    pub donor_name: Option<String>,
    pub user_id: Option<i32>,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DonationSettings {
    pub donation_goal: f64,
    pub donation_goal_period: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedDonationSettings {
    pub donation_goal: f64,
    pub donation_goal_period: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DonationStats {
    pub current_total: f64,
    pub goal: f64,
    pub period: String,
}
