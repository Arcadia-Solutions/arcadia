use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, Default, ToSchema)]
pub struct Invitation {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: NaiveDateTime,
    pub message: String,
    pub invitation_key: String,
    pub sender_id: i64,
    pub receiver_email: String,
    pub receiver_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SentInvitation {
    pub message: String,
    pub receiver_email: String,
}
