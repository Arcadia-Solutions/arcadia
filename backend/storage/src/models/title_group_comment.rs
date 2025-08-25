use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::user::UserLiteAvatar;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupComment {
    pub id: i64,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub title_group_id: i64,
    pub refers_to_torrent_id: Option<i64>,
    pub answers_to_comment_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedTitleGroupComment {
    pub content: String,
    pub title_group_id: i64,
    pub refers_to_torrent_id: Option<i64>,
    pub answers_to_comment_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupCommentHierarchy {
    pub id: i64,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub title_group_id: i64,
    pub refers_to_torrent_id: Option<i64>,
    pub answers_to_comment_id: Option<i64>,
    pub created_by: UserLiteAvatar,
}
