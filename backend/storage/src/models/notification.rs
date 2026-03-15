use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum NotificationEvent {
    ForumSubCategoryThread { user_ids: Vec<i32> },
    ForumThreadPost { user_ids: Vec<i32> },
    TitleGroupComment { user_ids: Vec<i32> },
    TitleGroupTorrent { user_ids: Vec<i32> },
    TorrentRequestComment { user_ids: Vec<i32> },
    StaffPmMessage { user_ids: Vec<i32> },
    Conversation { user_ids: Vec<i32> },
}

impl NotificationEvent {
    pub fn user_ids(&self) -> &[i32] {
        match self {
            Self::ForumSubCategoryThread { user_ids }
            | Self::ForumThreadPost { user_ids }
            | Self::TitleGroupComment { user_ids }
            | Self::TitleGroupTorrent { user_ids }
            | Self::TorrentRequestComment { user_ids }
            | Self::StaffPmMessage { user_ids }
            | Self::Conversation { user_ids } => user_ids,
        }
    }

    pub fn event_type(&self) -> &'static str {
        match self {
            Self::ForumSubCategoryThread { .. } => "forum_sub_category_thread",
            Self::ForumThreadPost { .. } => "forum_thread_post",
            Self::TitleGroupComment { .. } => "title_group_comment",
            Self::TitleGroupTorrent { .. } => "title_group_torrent",
            Self::TorrentRequestComment { .. } => "torrent_request_comment",
            Self::StaffPmMessage { .. } => "staff_pm_message",
            Self::Conversation { .. } => "conversation",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct NotificationForumSubCategoryThread {
    pub id: i64,
    pub forum_thread_id: i64,
    pub forum_thread_name: String,
    pub forum_sub_category_id: i32,
    pub forum_sub_category_name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub read_status: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct NotificationForumThreadPost {
    pub id: i64,
    pub forum_post_id: i64,
    pub forum_thread_id: i64,
    pub forum_thread_name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub read_status: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct NotificationTitleGroupComment {
    pub id: i64,
    pub title_group_comment_id: i64,
    pub title_group_id: i32,
    pub title_group_name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub read_status: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct NotificationTorrentRequestComment {
    pub id: i64,
    pub torrent_request_comment_id: i64,
    pub torrent_request_id: i64,
    pub title_group_name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub read_status: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct NotificationStaffPmMessage {
    pub id: i64,
    pub staff_pm_message_id: i64,
    pub staff_pm_id: i64,
    pub staff_pm_subject: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub read_status: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct NotificationCounts {
    pub announcements: i32,
    pub conversations: i32,
    pub forum_sub_category_threads: i32,
    pub forum_thread_posts: i32,
    pub title_group_comments: i32,
    pub staff_pm_messages: i32,
    pub torrent_request_comments: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Notifications {
    pub forum_sub_category_threads: Vec<NotificationForumSubCategoryThread>,
    pub forum_thread_posts: Vec<NotificationForumThreadPost>,
    pub title_group_comments: Vec<NotificationTitleGroupComment>,
    pub torrent_request_comments: Vec<NotificationTorrentRequestComment>,
    pub staff_pm_messages: Vec<NotificationStaffPmMessage>,
}
