use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct NotificationForumThreadPost {
    pub id: i64,
    pub forum_post_id: i64,
    pub forum_thread_id: i64,
    pub forum_thread_name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub read_status: bool,
}
