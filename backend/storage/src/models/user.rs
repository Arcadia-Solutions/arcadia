use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;

use super::peer::Peer;
use super::title_group::TitleGroupHierarchyLite;

// TODO: deserialize the settings field to a rust struct, currently doesn't seem possible
// https://github.com/launchbadge/sqlx/issues/3153#issuecomment-2798756953
// #[derive(Serialize, Deserialize, Debug, sqlx::Type)]
// #[sqlx(type_name = "item_detail_layout")]
// pub enum ItemDetailLayout {
// #[sqlx(rename = "header")]
// #[serde(rename = "header")]
//     Header,
// #[sqlx(rename = "sidebar_right")]
// #[serde(rename = "sidebar_right")]
//     SideBarRight,
// #[sqlx(rename = "sidebar_left")]
// #[serde(rename = "sidebar_left")]
//     SideBarLeft,
// }

// #[derive(Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
// pub struct SiteAppearanceSettings {
//     pub item_detail_layout: ItemDetailLayout,
// }

// #[derive(Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
// // #[sqlx(type_name = "user_settings")]
// pub struct UserSettings {
//     pub site_appearance: SiteAppearanceSettings,
// }

// causes errors
// https://github.com/launchbadge/sqlx/issues/3869

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
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
    pub ratio: f64,
    pub required_ratio: f64,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Utc>,
    pub class: UserClass,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub torrent_comments: i32,
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
    pub settings: serde_json::Value,
    pub warned: bool,
    pub banned: bool,
    pub staff_note: String,
    pub passkey_upper: i64,
    pub passkey_lower: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema, PartialEq, Eq)]
#[sqlx(type_name = "user_class_enum", rename_all = "lowercase")]
pub enum UserClass {
    Newbie,
    Staff,
    Tracker,
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
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
    pub class: UserClass,
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
    pub id: i64,
    pub username: String,
    pub avatar: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub description: String,
    pub uploaded: i64,
    pub real_uploaded: i64,
    pub downloaded: i64,
    pub real_downloaded: i64,
    pub ratio: f64,
    pub required_ratio: f64,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Utc>,
    pub class: UserClass,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub torrent_comments: i32,
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

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserLite {
    pub id: i64,
    pub username: String,
    pub warned: bool,
    pub banned: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserLiteAvatar {
    pub id: i64,
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
    pub unread_conversations_amount: u16,
    pub last_five_uploaded_torrents: Vec<TitleGroupHierarchyLite>,
    pub last_five_snatched_torrents: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicProfile {
    pub user: PublicUser,
    pub last_five_uploaded_torrents: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct UserWarning {
    pub id: i64,
    pub user_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub expires_at: Option<DateTime<Utc>>,
    pub reason: String,
    pub created_by_id: i64,
    pub ban: bool, // wether or not this warning bans the user
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedUserWarning {
    pub user_id: i64,
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
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedAPIKey {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserMinimal {
    pub id: i64,
    pub passkey_upper: i64,
    pub passkey_lower: i64,
}
