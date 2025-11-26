use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

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

#[derive(Debug, Serialize, ToSchema)]
pub struct TitleGroupTagSearchResult {
    pub name: String,
    pub synonyms: Vec<String>,
    pub id: i32,
}
