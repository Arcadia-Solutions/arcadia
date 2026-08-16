use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use arcadia_storage::models::{
    artist::{AffiliatedArtistHierarchy, ArtistRole},
    edition_group::UserCreatedEditionGroup,
    title_group::{ContentType, UserCreatedTitleGroup},
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ExternalDBData {
    pub title_group: Option<UserCreatedTitleGroup>,
    pub edition_group: Option<UserCreatedEditionGroup>,
    pub affiliated_artists: Vec<AffiliatedArtistHierarchy>, // pub series: UserCreatedSeries
    pub existing_title_group_id: Option<i32>,
}

/// An artist as returned by a scraper, before it has been created in the database.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ScrapedAffiliatedArtist {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub pictures: Vec<String>,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
}

/// The data returned by an external source plugin. Artists are given by name,
/// arcadia creates them and turns them into affiliated artists.
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ScrapedExternalData {
    pub title_group: Option<UserCreatedTitleGroup>,
    pub edition_group: Option<UserCreatedEditionGroup>,
    #[serde(default)]
    pub affiliated_artists: Vec<ScrapedAffiliatedArtist>,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct ExternalSource {
    pub id: String,
    pub placeholder: String,
    pub content_types: Vec<ContentType>,
}
