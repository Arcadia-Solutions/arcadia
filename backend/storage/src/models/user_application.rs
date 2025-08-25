use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, Display, sqlx::Type, ToSchema)]
#[sqlx(type_name = "user_application_status_enum", rename_all = "lowercase")]
pub enum UserApplicationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserApplication {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub body: String,
    pub email: String,
    pub referral: String,
    pub staff_note: String,
    pub status: UserApplicationStatus,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedUserApplication {
    pub body: String,
    pub email: String,
    pub referral: String,
}
