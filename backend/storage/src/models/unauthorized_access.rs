use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

use crate::models::common::OrderByDirection;

use super::user::{UserLiteAvatar, UserPermission};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UnauthorizedAccess {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub user: UserLiteAvatar,
    pub missing_permission: UserPermission,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema, Display)]
#[serde(rename_all = "snake_case")]
pub enum UnauthorizedAccessSortByColumn {
    CreatedAt,
    MissingPermission,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchUnauthorizedAccessQuery {
    pub user_id: Option<i32>,
    #[schema(value_type = String, format = DateTime)]
    #[param(value_type = String)]
    pub from_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    #[param(value_type = String)]
    pub to_date: DateTime<Utc>,
    pub permission: Option<UserPermission>,
    pub sort_by_column: UnauthorizedAccessSortByColumn,
    pub sort_by_direction: OrderByDirection,
    pub page: i64,
    pub page_size: i64,
}
