use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[sqlx(
    type_name = "snatched_torrent_bonus_points_transferred_to_enum",
    rename_all = "snake_case"
)]
pub enum SnatchedTorrentBonusPointsTransferredTo {
    Uploader,
    CurrentSeeders,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArcadiaSettingsForTracker {
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
    pub snatched_torrent_bonus_points_transferred_to:
        Option<SnatchedTorrentBonusPointsTransferredTo>,
}

impl ArcadiaSettingsForTracker {
    pub async fn from_database(db: &PgPool) -> Self {
        sqlx::query_as!(
            Self,
            r#"SELECT
                global_upload_factor,
                global_download_factor,
                snatched_torrent_bonus_points_transferred_to as "snatched_torrent_bonus_points_transferred_to: _"
            FROM arcadia_settings LIMIT 1"#
        )
        .fetch_one(db)
        .await
        .expect("could not get arcadia_settings")
    }
}
