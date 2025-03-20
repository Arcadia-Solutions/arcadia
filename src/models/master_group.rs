use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// TODO: This is a temporary doc, move it to a proper documentation when made
// MasterGroups are optional depending on the type of content
// They are not used for : Music, Movies, Books, TV Shows
// They are used for :
//   - Games (master group = game, title group = platform)
//
// When they are used, SimilarMasterGroups should be favored over SimilarTitleGroups for similarities/recommendations
//
// To make the search faster, as this is a heavily used function of the site
// we could have each torrent linked to their TitleGroup and MasterGroup in addition to the exising EditionGroup link
// this is a form of denormalization, but as torrents should not change groups (unless a rare edit), it might be a good consideration
//
// or adding some sort of cache/search engine on top, that has the data deserialized
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MasterGroup {
    pub id: i32,
    pub name: Option<String>,
    // pub name_aliases: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    // pub description: String,
    // pub original_language: String,
    // pub country_from: String,
    // pub covers: Option<Vec<String>>,
    // pub banners: Option<Vec<String>>,
    // pub fan_arts: Option<Vec<String>>,
    // pub category: Option<String>,
    // pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarMasterGroups {
    pub group_1: i32,
    pub group_2: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserCreatedMasterGroup {
    pub name: Option<String>,
    // pub name_aliases: Vec<String>,
    // pub description: String,
    // pub original_language: String,
    // pub country_from: String,
    // pub covers: Option<Vec<String>>,
    // pub banners: Option<Vec<String>>,
    // pub fan_arts: Option<Vec<String>>,
    // pub category: Option<String>,
    // pub tags: Vec<String>,
}
