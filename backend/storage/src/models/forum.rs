use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

use super::user::{UserLite, UserLiteAvatar};
use crate::utils::compute_diff;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumCategory {
    pub id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserCreatedForumCategory {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EditedForumCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumSubCategory {
    pub id: i32,
    pub forum_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i32,
    pub threads_amount: i64,
    pub posts_amount: i64,
    pub forbidden_classes: Vec<String>,
    pub new_threads_restricted: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserCreatedForumSubCategory {
    pub forum_category_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EditedForumSubCategory {
    pub id: i32,
    pub name: String,
    pub new_threads_restricted: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThread {
    pub id: i64,
    pub forum_sub_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i32,
    pub posts_amount: i64,
    pub pinned: bool,
    pub locked: bool,
    pub views_count: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct UserCreatedForumThread {
    pub forum_sub_category_id: i32,
    pub name: String,
    pub first_post: UserCreatedForumPost,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct EditedForumThread {
    pub id: i64,
    pub forum_sub_category_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPost {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i32,
    pub content: String,
    pub sticky: bool,
    pub locked: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct EditedForumPost {
    pub id: i64,
    pub content: String,
    pub sticky: bool,
    pub locked: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct UserCreatedForumPost {
    pub content: String,
    pub forum_thread_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteForumCategoryQuery {
    pub id: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteForumSubCategoryQuery {
    pub id: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteForumThreadQuery {
    pub id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteForumPostQuery {
    pub id: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumOverview {
    forum_categories: Vec<ForumCategoryHierarchy>,
    latest_posts_in_threads: Vec<ForumSearchResult>,
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
    pub new_threads_restricted: bool,
    pub is_allowed_poster: bool,
    pub latest_post_in_thread: Option<ForumThreadPostLite>,
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
    pub pinned: bool,
    pub locked: bool,
    pub views_count: i64,
    pub ever_opened: bool,
    pub has_new_posts: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThreadPostLite {
    pub id: i64,
    pub thread_id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by: UserLite,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThreadEnriched {
    pub id: i64,
    pub name: String,
    pub is_subscribed: bool,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i32,
    pub posts_amount: i64,
    pub pinned: bool,
    pub locked: bool,
    pub views_count: i64,
    pub forum_sub_category_name: String,
    pub forum_sub_category_id: i32,
    pub forum_category_name: String,
    pub forum_category_id: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPostHierarchy {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub created_by: UserLiteAvatar,
    pub content: String,
    pub sticky: bool,
    pub locked: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPostAndThreadName {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i32,
    pub content: String,
    pub sticky: bool,
    pub forum_thread_name: String,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetForumThreadPostsQuery {
    pub thread_id: i64,
    pub page: Option<u32>,
    pub page_size: u32,
    pub post_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumSearchResult {
    pub thread_name: String,
    pub thread_id: i64,
    pub post: String,
    pub post_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub post_created_at: DateTime<Utc>,
    pub post_created_by_id: i32,
    pub post_created_by_username: String,
    pub sub_category_name: String,
    pub sub_category_id: i32,
    pub category_name: String,
    pub category_id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct ForumSearchQuery {
    pub thread_name: Option<String>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ForumSubCategoryAllowedPoster {
    pub forum_sub_category_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetForumSubCategoryAllowedPostersQuery {
    pub forum_sub_category_id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct PinForumThread {
    pub pin: bool,
    pub thread_id: i64,
}

impl ForumCategory {
    pub fn diff(&self, edited: &EditedForumCategory) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}

impl ForumSubCategory {
    pub fn diff(&self, edited: &EditedForumSubCategory) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}

impl ForumThread {
    pub fn diff(&self, edited: &EditedForumThread) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}

impl ForumPost {
    pub fn diff(&self, edited: &EditedForumPost) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}

impl ForumThreadEnriched {
    pub fn diff(&self, edited: &EditedForumThread) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}
