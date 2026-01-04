use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::utils::compute_diff;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CssSheet {
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    pub name: String,
    pub css: String,
    pub preview_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CssSheetsEnriched {
    pub css_sheets: Vec<CssSheet>,
    pub default_sheet_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedCssSheet {
    pub name: String,
    pub css: String,
    pub preview_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedCssSheet {
    pub old_name: String,
    pub name: String,
    pub css: String,
    pub preview_image_url: String,
}

impl CssSheet {
    pub fn diff(&self, edited: &EditedCssSheet) -> Option<Value> {
        compute_diff(self, edited, &["old_name"])
    }
}
