use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::{IntoParams, ToSchema};

use super::{common::OrderByDirection, title_group::TitleGroupHierarchyLite};
use crate::utils::compute_diff;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema, Clone)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct EditedArtist {
    pub id: i64,
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

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
pub enum ArtistSearchOrderByColumn {
    #[serde(rename = "name")]
    #[strum(serialize = "name")]
    Name,
    #[serde(rename = "created_at")]
    #[strum(serialize = "created_at")]
    CreatedAt,
    #[serde(rename = "title_groups_amount")]
    #[strum(serialize = "title_groups_amount")]
    TitleGroupsAmount,
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct SearchArtistsQuery {
    pub name: Option<String>,
    pub page: u32,
    pub page_size: u32,
    pub order_by_column: ArtistSearchOrderByColumn,
    pub order_by_direction: OrderByDirection,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ArtistSearchResult {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    pub pictures: Vec<String>,
    pub title_groups_amount: i32,
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
    #[serde(rename = "creator")]
    #[sqlx(rename = "creator")]
    Creator,
    #[serde(rename = "performer")]
    #[sqlx(rename = "performer")]
    Performer,
    #[serde(rename = "presenter")]
    #[sqlx(rename = "presenter")]
    Presenter,
    #[serde(rename = "contributor")]
    #[sqlx(rename = "contributor")]
    Contributor,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtist {
    pub id: i64,
    pub title_group_id: i32,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>, // for example: name of the character the actor is playing
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtistLite {
    pub artist_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedAffiliatedArtist {
    pub title_group_id: i32,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ArtistAndTitleGroupsLite {
    pub artist: Artist,
    pub title_groups: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtistHierarchy {
    pub id: i64,
    pub title_group_id: i32,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i32,
    pub artist: Artist,
}

impl Artist {
    pub fn diff(&self, edited: &EditedArtist) -> Option<Value> {
        compute_diff(self, edited, &["id"])
    }
}
