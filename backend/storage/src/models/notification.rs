use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Hash, PartialEq, Eq)]
#[sqlx(type_name = "notification_reason_enum")]
pub enum NotificationReason {
    TorrentUploadedInSubscribedTitleGroup,
    SeedingTorrentDeleted,
    TitleGroupAddedForSubscribedArtist,
    ThreadAddedInSubscribedForumSubCategory,
    TitleGroupAddedInSubscribedCollage,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Notification {
    pub id: i64,
    pub created_at: DateTime<Local>,
    pub receiver_id: i64,
    pub reason: NotificationReason,
    pub message: Option<String>,
    pub read_status: bool,
    pub title_group_id: Option<i64>,
    pub torrent_id: Option<i64>,
    pub artist_id: Option<i64>,
    pub collage_id: Option<i64>,
    pub forum_thread_id: Option<i64>,
}
