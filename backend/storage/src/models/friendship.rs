use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use super::user::UserLiteAvatar;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "friend_request_status_enum", rename_all = "lowercase")]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct FriendRequest {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub status: FriendRequestStatus,
    pub message: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Friendship {
    pub id: i64,
    pub user1_id: i64,
    pub user2_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedFriendRequest {
    pub receiver_id: i64,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FriendRequestResponse {
    pub friend_request_id: i64,
    pub accept: bool, // true to accept, false to reject
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FriendRequestWithUser {
    pub id: i64,
    pub sender: UserLiteAvatar,
    pub receiver: UserLiteAvatar,
    pub status: FriendRequestStatus,
    pub message: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FriendshipWithUser {
    pub id: i64,
    pub friend: UserLiteAvatar,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FriendshipStatus {
    pub are_friends: bool,
    pub pending_request: Option<FriendRequestWithUser>,
}
