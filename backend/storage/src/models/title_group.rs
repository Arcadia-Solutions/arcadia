use chrono::{DateTime, Utc};
use musicbrainz_rs::entity::release_group::ReleaseGroupPrimaryType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::{
    artist::{AffiliatedArtistHierarchy, AffiliatedArtistLite, UserCreatedAffiliatedArtist},
    edition_group::{EditionGroupHierarchy, EditionGroupHierarchyLite, EditionGroupInfoLite},
    series::SeriesLite,
    title_group_comment::TitleGroupCommentHierarchy,
};
use crate::models::{
    entity::AffiliatedEntityHierarchy, torrent_request::TorrentRequestHierarchyLite,
};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "content_type_enum")]
pub enum ContentType {
    #[sqlx(rename = "movie")]
    #[serde(rename = "movie")]
    Movie,
    #[sqlx(rename = "video")]
    #[serde(rename = "video")]
    Video,
    #[sqlx(rename = "tv_show")]
    #[serde(rename = "tv_show")]
    TVShow,
    #[sqlx(rename = "music")]
    #[serde(rename = "music")]
    Music,
    #[sqlx(rename = "podcast")]
    #[serde(rename = "podcast")]
    Podcast,
    #[sqlx(rename = "software")]
    #[serde(rename = "software")]
    Software,
    #[sqlx(rename = "book")]
    #[serde(rename = "book")]
    Book,
    // aka SiteRip, but also includes packs of other content than website dumps (books, etc.)
    // this allows users to group content (when possible), which lowers the load on the tracker and makes the upload process faster (1 announce instead of multiple)
    #[sqlx(rename = "collection")]
    #[serde(rename = "collection")]
    Collection,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "platform_enum")]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Xbox,
}

// this is not to store the genre, but the format
#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "title_group_category_enum")]
pub enum TitleGroupCategory {
    //music
    Ep,
    Album, // includes "live album" as an "edition"
    Single,
    Soundtrack,
    Anthology,
    Compilation,
    Remix,
    Bootleg,
    Mixtape,
    ConcertRecording,
    DjMix,
    // movies
    FeatureFilm,
    ShortFilm,
    // tv shows
    // none ?
    // software
    Game,
    Program,
    // written documents (books and their derivatives)
    Illustrated, // includes mangas, comics, visual novels
    Periodical,  //includes newspapers, magazines
    Book,        // hardcover, etc
    Article,     // includes studies, theseis, essais, etc.
    Manual,      // includes guides, music sheets, etc. for physical and digital products
    // should be used as little as possible
    Other,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
pub enum ExternalDB {
    #[sqlx(rename = "tmdb")]
    #[serde(rename = "tmdb")]
    Tmdb,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicRating {
    pub service: ExternalDB,
    pub rating: f64,
    pub votes: i64,
}

// Every attribute is specific to the title,
// no specific information should be entered about the editions or the torrents
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroup {
    pub id: i64,
    pub master_group_id: Option<i64>, // only if master groups are needed for this type of content
    pub name: String,
    pub name_aliases: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub description: String,
    pub platform: Option<Platform>,
    pub original_language: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: DateTime<Utc>,
    pub tagline: Option<String>, // catchy sentence that represents the general idea of the title
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    #[schema(value_type = HashMap<String, HashMap<String, String>>)]
    pub embedded_links: Value, // {trailers: {name: link, name2: link2}, BehindTheScenes: {...}} (trailer, preview, etc.)
    pub category: Option<TitleGroupCategory>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,            // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    #[schema(value_type = Vec<PublicRating>)]
    pub public_ratings: Value, // {service: rating}
    pub series_id: Option<i64>,
    pub screenshots: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarTitleGroups {
    pub group_1: i64,
    pub group_2: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedTitleGroup {
    pub name: String,
    pub name_aliases: Vec<String>,
    pub description: String,
    pub original_language: Option<String>,
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    #[schema(value_type = HashMap<String, HashMap<String, String>>)]
    pub embedded_links: Value,
    // pub artists_affiliated: //(multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: Option<TitleGroupCategory>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,            // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub tagline: Option<String>,
    pub platform: Option<Platform>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: Option<DateTime<Utc>>,
    pub affiliated_artists: Vec<UserCreatedAffiliatedArtist>,
    pub series_id: Option<i64>,
    pub screenshots: Vec<String>,
    // one of them should be given, if master groups are required for this type of content
    pub master_group_id: Option<i64>,
    // pub master_group: Option<UserCreatedMasterGroup>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupHierarchyLite {
    pub id: i64,
    pub name: String,
    pub covers: Vec<String>,
    pub category: Option<TitleGroupCategory>,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: DateTime<Utc>,
    pub edition_groups: Vec<EditionGroupHierarchyLite>,
    pub affiliated_artists: Vec<AffiliatedArtistLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupLite {
    pub id: i64,
    pub name: String,
    pub content_type: ContentType,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: DateTime<Utc>,
    pub covers: Vec<String>,
    pub edition_groups: Vec<EditionGroupInfoLite>,
    pub platform: Option<Platform>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupInfoLite {
    pub id: i64,
    pub name: String,
    pub content_type: ContentType,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupAndAssociatedData {
    pub title_group: TitleGroup,
    pub edition_groups: Vec<EditionGroupHierarchy>,
    pub series: SeriesLite,
    pub affiliated_artists: Vec<AffiliatedArtistHierarchy>,
    pub affiliated_entities: Vec<AffiliatedEntityHierarchy>,
    pub title_group_comments: Vec<TitleGroupCommentHierarchy>,
    pub torrent_requests: Vec<TorrentRequestHierarchyLite>,
    pub is_subscribed: bool,
    pub in_same_master_group: Vec<TitleGroupLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditedTitleGroup {
    pub id: i64,
    pub master_group_id: Option<i64>,
    pub name: String,
    pub name_aliases: Vec<String>,
    pub description: String,
    pub platform: Option<Platform>,
    pub original_language: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: DateTime<Utc>,
    pub tagline: Option<String>,
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    #[schema(value_type = HashMap<String, HashMap<String, String>>)]
    pub embedded_links: Value,
    pub category: Option<TitleGroupCategory>,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    pub screenshots: Vec<String>,
}

pub fn create_default_title_group() -> UserCreatedTitleGroup {
    UserCreatedTitleGroup {
        name: String::from(""),
        name_aliases: Vec::new(),
        description: String::from(""),
        original_language: None,
        country_from: None,
        covers: vec!["".to_string()],
        external_links: vec!["".to_string()],
        embedded_links: json!({}),
        category: None,
        content_type: ContentType::Book,
        tags: Vec::new(),
        tagline: None,
        platform: None,
        original_release_date: Some(Utc::now()),
        affiliated_artists: Vec::new(),
        series_id: None,
        screenshots: Vec::new(),
        master_group_id: None,
    }
}

impl From<ReleaseGroupPrimaryType> for TitleGroupCategory {
    fn from(rg_type: ReleaseGroupPrimaryType) -> Self {
        match rg_type {
            ReleaseGroupPrimaryType::Album => TitleGroupCategory::Album,
            ReleaseGroupPrimaryType::Single => TitleGroupCategory::Single,
            ReleaseGroupPrimaryType::Ep => TitleGroupCategory::Ep,
            _ => TitleGroupCategory::Other,
        }
    }
}
