use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::user::UserLiteAvatar;
use crate::utils::compute_diff;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupComment {
    pub id: i64,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i32,
    pub title_group_id: i32,
    pub locked: bool,
    pub refers_to_torrent_id: Option<i32>,
    pub answers_to_comment_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedTitleGroupComment {
    pub content: String,
    pub title_group_id: i32,
    pub refers_to_torrent_id: Option<i32>,
    pub answers_to_comment_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedTitleGroupComment {
    pub content: String,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupCommentHierarchy {
    pub id: i64,
    pub content: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i32,
    pub title_group_id: i32,
    pub locked: bool,
    pub refers_to_torrent_id: Option<i32>,
    pub answers_to_comment_id: Option<i64>,
    pub created_by: UserLiteAvatar,
}

impl TitleGroupComment {
    pub fn diff(&self, edited: &EditedTitleGroupComment) -> Option<Value> {
        compute_diff(self, edited, &[])
    }
}
