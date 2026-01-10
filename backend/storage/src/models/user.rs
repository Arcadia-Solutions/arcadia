use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::Decode;
use utoipa::ToSchema;

use crate::models::common::OrderByDirection;

use super::peer::Peer;
use super::title_group::TitleGroupHierarchyLite;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub email: String,
    pub password_hash: String,
    #[schema(value_type = String, format = "0.0.0.0")]
    pub registered_from_ip: IpNetwork,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub uploaded: i64,
    pub real_uploaded: i64,
    pub downloaded: i64,
    pub real_downloaded: i64,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Utc>,
    pub class_name: String,
    pub class_locked: bool,
    pub permissions: Vec<UserPermission>,
    pub title_groups: i32,
    pub edition_groups: i32,
    pub torrents: i32,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub title_group_comments: i32,
    pub request_comments: i32,
    pub artist_comments: i64,
    pub seeding: i32,
    pub leeching: i32,
    pub snatched: i32,
    pub seeding_size: i64,
    pub requests_filled: i64,
    pub collages_started: i64,
    pub requests_voted: i64,
    pub average_seeding_time: i64, //in seconds
    pub invited: i64,
    pub invitations: i16,
    pub bonus_points: i64,
    pub freeleech_tokens: i32,
    pub warned: bool,
    pub banned: bool,
    pub staff_note: String,
    pub passkey: String,
    pub css_sheet_name: String,
    pub current_streak: i32,
    pub highest_streak: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq, Eq)]
#[sqlx(type_name = "user_permissions_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserPermission {
    UploadTorrent,
    DownloadTorrent,
    CreateTorrentRequest,
    ImmuneActivityPruning,
    EditTitleGroup,
    EditTitleGroupComment,
    EditEditionGroup,
    EditTorrent,
    EditArtist,
    DeleteArtist,
    EditCollage,
    DeleteCollage,
    EditSeries,
    EditTorrentRequest,
    EditForumPost,
    EditForumThread,
    EditForumSubCategory,
    EditForumCategory,
    CreateForumCategory,
    CreateForumSubCategory,
    CreateForumThread,
    CreateForumPost,
    DeleteForumCategory,
    DeleteForumSubCategory,
    DeleteForumThread,
    DeleteForumPost,
    SendPm,
    CreateCssSheet,
    EditCssSheet,
    ReadStaffPm,
    ReplyStaffPm,
    ResolveStaffPm,
    UnresolveStaffPm,
    DeleteTitleGroupTag,
    EditTitleGroupTag,
    DeleteTorrent,
    SetTorrentStaffChecked,
    GetUserApplication,
    UpdateUserApplication,
    WarnUser,
    BanUser,
    EditUser,
    CreateWikiArticle,
    EditWikiArticle,
    CreateUserClass,
    EditUserClass,
    DeleteUserClass,
    EditUserPermissions,
    LockUserClass,
    ChangeUserClass,
    EditArcadiaSettings,
    CreateDonation,
    EditDonation,
    DeleteDonation,
    SearchDonation,
    SearchUnauthorizedAccess,
    SearchUserEditChangeLogs,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub password_verify: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedUser {
    pub avatar: Option<String>,
    pub email: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub uploaded: i64,
    pub real_uploaded: i64,
    pub downloaded: i64,
    pub real_downloaded: i64,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Utc>,
    pub class_name: String,
    pub class_locked: bool,
    pub title_groups: i32,
    pub edition_groups: i32,
    pub torrents: i32,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub title_group_comments: i32,
    pub request_comments: i32,
    pub artist_comments: i64,
    pub seeding: i32,
    pub leeching: i32,
    pub snatched: i32,
    pub seeding_size: i64,
    pub requests_filled: i64,
    pub collages_started: i64,
    pub requests_voted: i64,
    pub average_seeding_time: i64,
    pub invited: i64,
    pub invitations: i16,
    pub bonus_points: i64,
    pub banned: bool,
    pub warned: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema, Decode)]
pub struct UserLite {
    pub id: i32,
    pub username: String,
    pub warned: bool,
    pub banned: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserLiteAvatar {
    pub id: i32,
    pub username: String,
    pub banned: bool,
    pub avatar: Option<String>,
    pub warned: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub user: User,
    pub peers: Vec<Peer>,
    pub user_warnings: Vec<UserWarning>,
    pub unread_conversations_amount: u32,
    pub unread_notifications_amount_forum_thread_posts: u32,
    pub unread_notifications_amount_title_group_comments: u32,
    pub last_five_uploaded_torrents: Vec<TitleGroupHierarchyLite>,
    pub last_five_snatched_torrents: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicProfile {
    pub user: PublicUser,
    pub last_five_uploaded_torrents: Vec<TitleGroupHierarchyLite>,
    pub last_five_snatched_torrents: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct UserWarning {
    pub id: i64,
    pub user_id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub expires_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub created_by_id: i32,
    pub ban: bool, // wether or not this warning bans the user
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedUserWarning {
    pub user_id: i32,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub expires_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub ban: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]

pub struct APIKey {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub value: String,
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedAPIKey {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserMinimal {
    pub id: i32,
    pub passkey: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserSettings {
    pub css_sheet_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserClass {
    pub name: String,
    pub new_permissions: Vec<UserPermission>,
    pub automatic_promotion: bool,
    pub automatic_demotion: bool,
    pub promotion_allowed_while_warned: bool,
    pub previous_user_class: Option<String>,
    pub required_account_age_in_days: i32,
    pub required_ratio: f64,
    pub required_torrent_uploads: i32,
    pub required_torrent_uploads_in_unique_title_groups: i32,
    pub required_uploaded: i64,
    pub required_torrent_snatched: i32,
    pub required_downloaded: i64,
    pub required_forum_posts: i32,
    pub required_forum_posts_in_unique_threads: i32,
    pub required_title_group_comments: i32,
    pub required_seeding_size: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedUserClass {
    pub name: String,
    pub new_permissions: Vec<UserPermission>,
    pub automatic_promotion: bool,
    pub automatic_demotion: bool,
    pub promotion_allowed_while_warned: bool,
    pub previous_user_class: Option<String>,
    pub required_account_age_in_days: i32,
    pub required_ratio: f64,
    pub required_torrent_uploads: i32,
    pub required_torrent_uploads_in_unique_title_groups: i32,
    pub required_uploaded: i64,
    pub required_torrent_snatched: i32,
    pub required_downloaded: i64,
    pub required_forum_posts: i32,
    pub required_forum_posts_in_unique_threads: i32,
    pub required_title_group_comments: i32,
    pub required_seeding_size: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedUserClass {
    pub name: String,
    pub new_permissions: Vec<UserPermission>,
    pub automatic_promotion: bool,
    pub automatic_demotion: bool,
    pub promotion_allowed_while_warned: bool,
    pub previous_user_class: Option<String>,
    pub required_account_age_in_days: i32,
    pub required_ratio: f64,
    pub required_torrent_uploads: i32,
    pub required_torrent_uploads_in_unique_title_groups: i32,
    pub required_uploaded: i64,
    pub required_torrent_snatched: i32,
    pub required_downloaded: i64,
    pub required_forum_posts: i32,
    pub required_forum_posts_in_unique_threads: i32,
    pub required_title_group_comments: i32,
    pub required_seeding_size: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatedUserPermissions {
    pub permissions: Vec<UserPermission>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserClassLockStatus {
    pub class_locked: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserClassChange {
    pub class_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteUserClass {
    pub target_class_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserSearchResult {
    pub id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub class_name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Utc>,
    pub uploaded: i64,
    pub downloaded: i64,
    pub torrents: i32,
    pub title_groups: i32,
    pub title_group_comments: i32,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub warned: bool,
    pub banned: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, utoipa::IntoParams)]
pub struct SearchUsersQuery {
    pub username: Option<String>,
    pub order_by: UserSearchOrderBy,
    pub order_by_direction: OrderByDirection,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, strum::Display)]
pub enum UserSearchOrderBy {
    #[serde(rename = "username")]
    #[strum(serialize = "username")]
    Username,
    #[serde(rename = "created_at")]
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[serde(rename = "uploaded")]
    #[strum(serialize = "uploaded")]
    Uploaded,
    #[serde(rename = "downloaded")]
    #[strum(serialize = "downloaded")]
    Downloaded,
    #[serde(rename = "torrents")]
    #[strum(serialize = "torrents")]
    Torrents,
    #[serde(rename = "title_groups")]
    #[strum(serialize = "title_groups")]
    TitleGroups,
    #[serde(rename = "title_group_comments")]
    #[strum(serialize = "title_group_comments")]
    TitleGroupComments,
    #[serde(rename = "forum_posts")]
    #[strum(serialize = "forum_posts")]
    ForumPosts,
    #[serde(rename = "forum_threads")]
    #[strum(serialize = "forum_threads")]
    ForumThreads,
}
