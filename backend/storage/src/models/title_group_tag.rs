use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

use crate::models::{common::OrderByDirection, user::UserLite};
use crate::utils::compute_diff;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct TitleGroupTag {
    pub id: i32,
    pub name: String,
    pub synonyms: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreatedTitleGroupTag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedTitleGroupTag {
    pub id: i32,
    pub name: String,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TitleGroupTagLite {
    pub name: String,
    pub synonyms: Vec<String>,
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct TitleGroupTagEnriched {
    pub id: i32,
    pub name: String,
    pub synonyms: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by: UserLite,
    pub uses: i32,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<UserLite>,
    pub deletion_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
pub enum TitleGroupTagSearchOrderByColumn {
    #[serde(rename = "created_at")]
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[serde(rename = "uses")]
    #[strum(serialize = "uses")]
    Uses,
    #[serde(rename = "name")]
    #[strum(serialize = "name")]
    Name,
    #[serde(rename = "created_by")]
    #[strum(serialize = "created_by")]
    CreatedBy,
    #[serde(rename = "deleted_at")]
    #[strum(serialize = "deleted_at")]
    DeletedAt,
    #[serde(rename = "deleted_by")]
    #[strum(serialize = "deleted_by")]
    DeletedBy,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct SearchTitleGroupTagsQuery {
    pub name: String,
    /// when true, the deleted tags are returned alongside the live ones
    pub show_deleted: bool,
    // pagination and ordering
    pub page: u32,
    pub page_size: u32,
    pub order_by_column: TitleGroupTagSearchOrderByColumn,
    pub order_by_direction: OrderByDirection,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteTitleGroupTagRequest {
    pub id: i32,
    pub deletion_reason: String,
}

impl TitleGroupTag {
    pub fn diff(&self, edited: &EditedTitleGroupTag) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}
