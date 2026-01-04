use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::user::UserLite;
use crate::utils::compute_diff;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WikiArticle {
    pub id: i64,
    pub title: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub updated_by_id: i32,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedWikiArticle {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WikiArticleHierarchy {
    pub id: i64,
    pub title: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by: UserLite,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub updated_by: UserLite,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditedWikiArticle {
    pub id: i64,
    pub title: String,
    pub body: String,
}

impl WikiArticle {
    pub fn diff(&self, edited: &EditedWikiArticle) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}
