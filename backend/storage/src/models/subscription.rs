use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Subscription {
    pub id: i64,
    pub subscribed_at: DateTime<Local>,
    pub subscriber_id: i64,
    pub title_group_id: i64,
    pub artist_id: i64,
    pub forum_thread_id: i64,
    pub forum_sub_category_id: i64,
}
