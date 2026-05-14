use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, sqlx::Type, ToSchema)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "site_highlight_item_type_enum", rename_all = "snake_case")]
pub enum SiteHighlightItemType {
    TitleGroup,
    Series,
    Artist,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SiteHighlight {
    pub id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    pub alias: String,
    pub item_type: SiteHighlightItemType,
    pub item_id: i64,
    pub forum_thread_id: i64,
    pub enabled: bool,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SiteHighlightForHome {
    pub id: i32,
    pub alias: String,
    pub item_type: SiteHighlightItemType,
    pub item_id: i64,
    pub item_image: Option<String>,
    pub forum_thread_id: i64,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSiteHighlight {
    pub alias: String,
    pub item_type: SiteHighlightItemType,
    pub item_id: i64,
    pub forum_thread_id: i64,
    pub enabled: bool,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EditSiteHighlight {
    pub alias: Option<String>,
    pub item_type: Option<SiteHighlightItemType>,
    pub item_id: Option<i64>,
    pub forum_thread_id: Option<i64>,
    pub enabled: Option<bool>,
    pub position: Option<i32>,
    pub remove_previous_related_thread: bool,
}
