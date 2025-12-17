use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArcadiaSettingsForTracker {
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
}

impl ArcadiaSettingsForTracker {
    pub async fn from_database(db: &PgPool) -> Self {
        sqlx::query_as!(
            Self,
            "SELECT global_upload_factor, global_download_factor FROM arcadia_settings LIMIT 1"
        )
        .fetch_one(db)
        .await
        .expect("could not get arcadia_settings")
    }
}
