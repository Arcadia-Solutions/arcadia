use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

use super::title_group::TitleGroupHierarchyLite;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Series {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i32,
    pub covers: Vec<String>,
    pub banners: Option<Vec<String>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedSeries {
    pub name: String,
    pub description: String,
    pub covers: Vec<String>,
    pub banners: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SeriesAndTitleGroupHierarchyLite {
    pub series: Series,
    pub title_groups: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SeriesLite {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SeriesSearchResult {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i32,
    pub covers: Vec<String>,
    pub banners: Option<Vec<String>>,
    pub tags: Vec<String>,
    pub title_groups_amount: i64,
    // pub newest_title_group: Option<TitleGroupInfoLite>,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct SearchSeriesQuery {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SeriesSearchResponse {
    pub results: Vec<SeriesSearchResult>,
    pub total_items: i64,
}
