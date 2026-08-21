use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Emoji {
    pub id: i32,
    pub name: String,
    /// Set when the emoji is a unicode character, null when it is a stored image.
    pub unicode_character: Option<String>,
    /// Seconds since the unix epoch of the last edit, used to build a cache busting image URL.
    pub image_version: i64,
    pub sort_order: i16,
    /// Disabled emojis are hidden from the reaction picker, but existing reactions using them
    /// are still returned and counted.
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EmojiUsage {
    pub emoji_id: i32,
    pub reactions_amount: i64,
}

/// The raw bytes of a custom emoji, served by the image endpoint.
#[derive(Debug, FromRow)]
pub struct EmojiImage {
    pub image: Vec<u8>,
    pub image_mime_type: String,
    /// Seconds since the unix epoch of the last edit, the same value exposed as `image_version`
    /// on `Emoji`, used to build the response `ETag`.
    pub image_version: i64,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct DeleteEmojiQuery {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ReorderEmojiEntry {
    pub id: i32,
    pub sort_order: i16,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ReorderEmojis {
    pub emojis: Vec<ReorderEmojiEntry>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EmojiEnabledUpdate {
    pub id: i32,
    pub enabled: bool,
}
