use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use arcadia_storage::models::{
    edition_group::UserCreatedEditionGroup, title_group::UserCreatedTitleGroup,
};

pub mod comic_vine;
pub mod isbn;
pub mod musicbrainz;
pub mod tmdb;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ExternalDBData {
    pub title_group: Option<UserCreatedTitleGroup>,
    pub edition_group: Option<UserCreatedEditionGroup>,
    // pub series: UserCreatedSeries
}
