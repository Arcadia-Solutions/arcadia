use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use arcadia_storage::models::{
    edition_group::UserCreatedEditionGroup, title_group::UserCreatedTitleGroup,
};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ExternalDBData {
    pub title_group: Option<UserCreatedTitleGroup>,
    pub edition_group: Option<UserCreatedEditionGroup>,
    // pub series: UserCreatedSeries
}
