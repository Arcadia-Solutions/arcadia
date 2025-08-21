use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};
use utoipa::ToSchema;

use super::torrent::{TorrentHierarchy, TorrentHierarchyLite};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "source_enum")]
pub enum Source {
    #[sqlx(rename = "CD")]
    #[serde(rename = "CD")]
    Cd,
    #[sqlx(rename = "DVD5")]
    #[serde(rename = "DVD5")]
    Dvd5,
    #[sqlx(rename = "DVD9")]
    #[serde(rename = "DVD9")]
    Dvd9,
    Vinyl,
    Web,
    Soundboard,
    #[sqlx(rename = "SACD")]
    #[serde(rename = "SACD")]
    Sacd,
    #[sqlx(rename = "DAT")]
    #[serde(rename = "DAT")]
    Dat,
    Cassette,
    #[sqlx(rename = "Blu-Ray")]
    #[serde(rename = "Blu-Ray")]
    BluRay,
    LaserDisc,
    #[sqlx(rename = "HD-DVD")]
    #[serde(rename = "HD-DVD")]
    Hddvd,
    #[sqlx(rename = "HDTV")]
    #[serde(rename = "HDTV")]
    Hdtv,
    #[sqlx(rename = "PDTV")]
    #[serde(rename = "PDTV")]
    Pdtv,
    #[sqlx(rename = "TV")]
    #[serde(rename = "TV")]
    Tv,
    #[sqlx(rename = "VHS")]
    #[serde(rename = "VHS")]
    Vhs,
    Mixed,
    #[sqlx(rename = "Physical Book")]
    #[serde(rename = "Physical Book")]
    PhysicalBook,
}

// This represents encodes/transcodes of the same edition.
// All the torrents in it originate from the same source.
// It is independant people that produced multiple encodes/transcodes alongside the original one(s).
// Every attribute is specific to the edition, no information should be entered about the torrents or the title
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditionGroup {
    pub id: i64,
    pub title_group_id: i64,
    pub name: Option<String>, // edition name, not title name, (also, for Collections, includes the optional subscription level/tier)
    #[schema(value_type = String, format = DateTime)]
    pub release_date: DateTime<Utc>, // public release, (also, for Collections, date of the last (chronologically) item included)
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>, // database entry creation
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub description: Option<String>, // specific to the edition
    pub distributor: Option<String>, // web: [web stores/distributors], physical: [shop if specific edition ?]
    pub covers: Vec<String>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub source: Option<Source>,
    // this information will appea in the "title bar" of the edition
    // for collections : (date_from: first item date, first_item: numer/name of the first item, last_item: number/name of the last item)
    // for music: (label, catalogue_number)
    // for books: (format: ebook/audiobook, isbn)
    #[schema(value_type = HashMap<String, String>)]
    pub additional_information: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedEditionGroup {
    pub name: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub release_date: DateTime<Utc>,
    pub description: Option<String>,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    pub source: Option<Source>,
    #[schema(value_type = HashMap<String, String>)]
    pub additional_information: Option<Value>,
    pub title_group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditionGroupHierarchyLite {
    pub id: i64,
    pub title_group_id: i64,
    pub name: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub release_date: DateTime<Utc>,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub source: Option<Source>,
    #[schema(value_type = HashMap<String, String>)]
    pub additional_information: Option<Json<Value>>,
    pub torrents: Vec<TorrentHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditionGroupHierarchy {
    pub id: i64,
    pub title_group_id: i64,
    pub name: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub release_date: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub description: Option<String>,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    pub source: Option<Source>,
    #[schema(value_type = HashMap<String, String>)]
    pub additional_information: Option<Json<Value>>,
    pub torrents: Vec<TorrentHierarchy>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditionGroupInfoLite {
    pub id: i64,
    pub name: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub release_date: DateTime<Utc>,
    pub distributor: Option<String>,
    pub source: Option<Source>,
    #[schema(value_type = HashMap<String, String>)]
    pub additional_information: Option<Json<Value>>,
}

pub fn create_default_edition_group() -> UserCreatedEditionGroup {
    UserCreatedEditionGroup {
        name: None,
        release_date: Utc::now(),
        description: None,
        distributor: None,
        covers: vec![],
        external_links: vec![],
        source: None,
        additional_information: None,
        title_group_id: 0,
    }
}
