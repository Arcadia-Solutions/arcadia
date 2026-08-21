use crate::models::user::UserLite;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// One emoji used on one piece of content, with its count.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ContentReaction {
    pub emoji_id: i32,
    pub emoji_name: String,
    pub emoji_unicode_character: Option<String>,
    pub emoji_image_version: i64,
    pub amount: i64,
    pub reacted_by_current_user: bool,
}

/// The users who used one emoji on one forum post.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ForumPostReactionUsers {
    pub emoji_id: i32,
    /// Capped at 100 entries, oldest reaction first.
    pub users: Vec<UserLite>,
    /// The real amount of reactions, which can be greater than `users.len()`.
    pub total_amount: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreatedForumPostReaction {
    pub forum_post_id: i64,
    pub emoji_id: i32,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct DeleteForumPostReactionQuery {
    pub forum_post_id: i64,
    pub emoji_id: i32,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetForumPostReactionUsersQuery {
    pub forum_post_id: i64,
}
