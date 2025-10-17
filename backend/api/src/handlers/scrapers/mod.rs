use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use arcadia_storage::models::{
    artist::AffiliatedArtistHierarchy, edition_group::UserCreatedEditionGroup,
    title_group::UserCreatedTitleGroup,
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ExternalDBData {
    pub title_group: Option<UserCreatedTitleGroup>,
    pub edition_group: Option<UserCreatedEditionGroup>,
    pub affiliated_artists: Vec<AffiliatedArtistHierarchy>, // pub series: UserCreatedSeries
    pub existing_title_group_id: Option<i32>,
}
