use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::user::UserLite;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WikiArticle {
    pub id: i64,
    pub title: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub updated_by_id: i64,
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
