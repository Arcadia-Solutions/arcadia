use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "collage_category_enum")]
pub enum CollageCategory {
    Personal,
    #[sqlx(rename = "Staff Picks")]
    #[serde(rename = "Staff Picks")]
    StaffPicks,
    External, // replicates a collage from somewhere else (yt playlist, magazine's picks, etc.)
    Theme,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "collage_type_enum")]
pub enum CollageSection {
    Artist,
    Entity,
    Title,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Collage {
    pub id: i64,
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub name: String,
    pub covers: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category: CollageCategory,
    pub section: CollageSection,
}
