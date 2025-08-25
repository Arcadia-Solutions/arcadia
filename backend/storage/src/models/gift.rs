use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Gift {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub sent_at: DateTime<Utc>,
    pub message: String,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub bonus_points: i64,
    pub freeleech_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedGift {
    pub message: String,
    pub receiver_id: i64,
    pub bonus_points: i64,
    pub freeleech_tokens: i32,
}
