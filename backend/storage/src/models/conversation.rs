use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::models::user::UserLite;

use super::user::UserLiteAvatar;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Conversation {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub sender_id: i64,
    pub receiver_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub sender_last_seen_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub receiver_last_seen_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedConversation {
    pub subject: String,
    pub receiver_id: i64,
    pub first_message: UserCreatedConversationMessage,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ConversationMessage {
    pub id: i64,
    pub conversation_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
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
    pub sender_id: i64,
    pub receiver_id: i64,
    pub correspondant: UserLite,
    #[schema(value_type = String, format = DateTime)]
    pub sender_last_seen_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub receiver_last_seen_at: Option<DateTime<Utc>>,
    pub last_message: ConversationMessageHierarchyLite,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConversationsOverview {
    conversations: Vec<ConversationOverview>,
}
