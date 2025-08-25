use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::title_group::TitleGroupHierarchyLite;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub description: String,
    pub pictures: Vec<String>,
    pub title_groups_amount: i32,
    pub edition_groups_amount: i32,
    pub torrents_amount: i32,
    pub seeders_amount: i32,
    pub leechers_amount: i32,
    pub snatches_amount: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SimilarArtists {
    pub artist_1_id: i64,
    pub artist_2_id: i64,
}

#[derive(Debug, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedArtist {
    pub name: String,
    pub description: String,
    pub pictures: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ArtistLite {
    pub id: i64,
    pub name: String,
    pub pictures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "artist_role_enum")]
pub enum ArtistRole {
    #[serde(rename = "main")]
    #[sqlx(rename = "main")]
    Main,
    #[serde(rename = "guest")]
    #[sqlx(rename = "guest")]
    Guest,
    #[serde(rename = "producer")]
    #[sqlx(rename = "producer")]
    Producer,
    #[serde(rename = "director")]
    #[sqlx(rename = "director")]
    Director,
    #[serde(rename = "cinematographer")]
    #[sqlx(rename = "cinematographer")]
    Cinematographer,
    #[serde(rename = "actor")]
    #[sqlx(rename = "actor")]
    Actor,
    #[serde(rename = "writer")]
    #[sqlx(rename = "writer")]
    Writer,
    #[serde(rename = "composer")]
    #[sqlx(rename = "composer")]
    Composer,
    #[serde(rename = "remixer")]
    #[sqlx(rename = "remixer")]
    Remixer,
    #[serde(rename = "conductor")]
    #[sqlx(rename = "conductor")]
    Conductor,
    #[serde(rename = "dj_compiler")]
    #[sqlx(rename = "dj_compiler")]
    DjCompiler,
    #[serde(rename = "arranger")]
    #[sqlx(rename = "arranger")]
    Arranger,
    #[serde(rename = "host")]
    #[sqlx(rename = "host")]
    Host,
    #[serde(rename = "author")]
    #[sqlx(rename = "author")]
    Author,
    #[serde(rename = "illustrator")]
    #[sqlx(rename = "illustrator")]
    Illustrator,
    #[serde(rename = "editor")]
    #[sqlx(rename = "editor")]
    Editor,
    #[serde(rename = "developer")]
    #[sqlx(rename = "developer")]
    Developer,
    #[serde(rename = "designer")]
    #[sqlx(rename = "designer")]
    Designer,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtist {
    pub id: i64,
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>, // for example: name of the character the actor is playing
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtistLite {
    pub artist_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedAffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ArtistAndTitleGroupsLite {
    // used for the API doc, but not sure why it's considered dead code
    #[allow(dead_code)]
    pub artist: Artist,
    #[allow(dead_code)]
    pub title_groups: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtistHierarchy {
    pub id: i64,
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub artist: Artist,
}
