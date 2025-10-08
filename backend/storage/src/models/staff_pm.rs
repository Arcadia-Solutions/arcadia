use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StaffPm {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub created_by_id: i32,
    pub resolved: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StaffPmMessage {
    pub id: i64,
    pub staff_pm_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedStaffPm {
    pub subject: String,
    pub first_message: UserCreatedStaffPmMessage,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedStaffPmMessage {
    pub staff_pm_id: i64,
    pub content: String,
}

use super::user::{UserLite, UserLiteAvatar};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StaffPmMessageHierarchyLite {
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by: UserLite,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StaffPmOverview {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub created_by: UserLite,
    pub resolved: bool,
    pub last_message: StaffPmMessageHierarchyLite,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StaffPmMessageHierarchy {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by: UserLiteAvatar,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StaffPmHierarchy {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub created_by: UserLiteAvatar,
    pub resolved: bool,
    pub messages: Vec<StaffPmMessageHierarchy>,
}
