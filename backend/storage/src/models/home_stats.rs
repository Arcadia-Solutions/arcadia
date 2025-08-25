use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct HomeStats {
    pub enabled_users: i64,
    pub users_active_today: i64,
    pub users_active_this_week: i64,
    pub users_active_this_month: i64,
    pub torrents: i64,
    pub torrents_uploaded_today: i64,
    pub titles: i64,
    pub artists: i64,
    pub entities: i64,
}
