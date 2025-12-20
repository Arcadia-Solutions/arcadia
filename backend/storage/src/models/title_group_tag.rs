use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

use crate::models::{common::OrderByDirection, user::UserLite};

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

#[derive(Debug, Deserialize, ToSchema)]
pub struct EditedTitleGroupTag {
    pub id: i32,
    pub name: String,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
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
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct SearchTitleGroupTagsQuery {
    pub name: String,
    // pagination and ordering
    pub page: u32,
    pub page_size: u32,
    pub order_by_column: TitleGroupTagSearchOrderByColumn,
    pub order_by_direction: OrderByDirection,
}
