use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

use crate::models::common::OrderByDirection;
use crate::models::user::UserLite;

use super::user::UserLiteAvatar;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Conversation {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub sender_id: i32,
    pub receiver_id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub sender_last_seen_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub receiver_last_seen_at: Option<DateTime<Utc>>,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedConversation {
    pub subject: String,
    pub receiver_id: i32,
    pub first_message: UserCreatedConversationMessage,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ConversationMessage {
    pub id: i64,
    pub conversation_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedConversationMessage {
    pub conversation_id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConversationMessageHierarchy {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by: UserLiteAvatar,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConversationHierarchy {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub sender: UserLiteAvatar,
    pub receiver: UserLiteAvatar,
    #[schema(value_type = String, format = DateTime)]
    pub sender_last_seen_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub receiver_last_seen_at: Option<DateTime<Utc>>,
    pub locked: bool,
    pub messages: Vec<ConversationMessageHierarchy>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConversationMessageHierarchyLite {
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by: UserLite,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConversationOverview {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub correspondant: UserLite,
    #[schema(value_type = String, format = DateTime)]
    pub sender_last_seen_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub receiver_last_seen_at: Option<DateTime<Utc>>,
    pub locked: bool,
    pub last_message: ConversationMessageHierarchyLite,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConversationsOverview {
    conversations: Vec<ConversationOverview>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ConversationSearchQuery {
    pub search_term: Option<String>,
    pub search_titles_only: bool,
    pub page: u32,
    pub page_size: u32,
    pub user_id: Option<i32>,
    pub order_by_column: ConversationSearchOrderByColumn,
    pub order_by_direction: OrderByDirection,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ConversationSearchOrderByColumn {
    CreatedAt,
    LastMessage,
    MessagesAmount,
    Subject,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ConversationSearchResult {
    pub conversation_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub conversation_created_at: DateTime<Utc>,
    pub subject: String,
    pub sender_id: i32,
    pub receiver_id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub sender_last_seen_at: DateTime<Utc>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub receiver_last_seen_at: Option<DateTime<Utc>>,
    pub locked: bool,
    pub correspondant_id: i32,
    pub correspondant_username: String,
    pub correspondant_warned: bool,
    pub correspondant_banned: bool,
    pub sender_username: String,
    pub sender_warned: bool,
    pub sender_banned: bool,
    pub receiver_username: String,
    pub receiver_warned: bool,
    pub receiver_banned: bool,
    pub messages_amount: i64,
    #[schema(value_type = String, format = DateTime)]
    pub last_message_created_at: DateTime<Utc>,
    pub last_message_created_by_id: i32,
    pub last_message_created_by_username: String,
}
