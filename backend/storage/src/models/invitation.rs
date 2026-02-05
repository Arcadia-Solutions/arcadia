use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

use crate::models::common::OrderByDirection;
use crate::models::user::UserLiteAvatar;

#[derive(Debug, Serialize, Deserialize, FromRow, Default, ToSchema)]
pub struct Invitation {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: DateTime<Local>,
    pub message: String,
    pub invitation_key: String,
    pub sender_id: i32,
    pub receiver_email: String,
    pub receiver_id: Option<i32>,
    pub user_application_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SentInvitation {
    pub message: String,
    pub receiver_email: String,
    pub user_application_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
pub enum InvitationSearchOrderByColumn {
    #[serde(rename = "created_at")]
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[serde(rename = "receiver_username")]
    #[strum(serialize = "receiver_username")]
    ReceiverUsername,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct SearchSentInvitationsQuery {
    pub receiver_username: Option<String>,
    pub page: u32,
    pub page_size: u32,
    pub order_by_column: InvitationSearchOrderByColumn,
    pub order_by_direction: OrderByDirection,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Default, ToSchema)]
pub struct InvitationHierarchy {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: DateTime<Local>,
    pub message: String,
    pub invitation_key: String,
    pub sender_id: i32,
    pub receiver_email: String,
    pub receiver: Option<UserLiteAvatar>,
    pub user_application_id: Option<i64>,
}
