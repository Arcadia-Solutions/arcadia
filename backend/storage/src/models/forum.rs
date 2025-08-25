use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::user::{UserLite, UserLiteAvatar};

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumCategory {
    pub id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumSubCategory {
    pub id: i32,
    pub forum_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub threads_amount: i64,
    pub posts_amount: i64,
    pub forbidden_classes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThread {
    pub id: i64,
    pub forum_sub_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub posts_amount: i64,
    pub sticky: bool,
    pub locked: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct UserCreatedForumThread {
    pub forum_sub_category_id: i32,
    pub name: String,
    pub first_post: UserCreatedForumPost,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPost {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub content: String,
    pub sticky: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct UserCreatedForumPost {
    pub content: String,
    pub forum_thread_id: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumOverview {
    forum_categories: Vec<ForumCategoryHierarchy>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumCategoryHierarchy {
    pub id: i32,
    pub name: String,
    pub sub_categories: Vec<ForumSubCategoryHierarchy>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumCategoryLite {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumSubCategoryHierarchy {
    pub id: i32,
    pub name: String,
    pub threads_amount: i64,
    pub posts_amount: i64,
    pub forbidden_classes: Vec<String>,
    pub latest_post_in_thread: ForumThreadPostLite,
    pub threads: Option<Vec<ForumThreadHierarchy>>,
    pub category: ForumCategoryLite,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThreadHierarchy {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by: UserLite,
    pub latest_post: ForumThreadPostLite,
    pub posts_amount: i64,
    pub sticky: bool,
    pub locked: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThreadPostLite {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by: UserLite,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThreadAndPosts {
    pub id: i64,
    pub forum_sub_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub posts_amount: i64,
    pub sticky: bool,
    pub locked: bool,
    pub posts: Vec<ForumPostHierarchy>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPostHierarchy {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by: UserLiteAvatar,
    pub content: String,
    pub sticky: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPostAndThreadName {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub content: String,
    pub sticky: bool,
    pub forum_thread_name: String,
}
