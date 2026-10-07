use crate::models::torrent_stats::TitleGroupsPerReleaseYearDataPoint;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum::Display;
use utoipa::ToSchema;

/// The kind of catalog whose title groups the stats aggregate over.
#[derive(Debug, Clone, Copy, Display)]
#[strum(serialize_all = "snake_case")]
pub enum CatalogKind {
    Artist,
    Series,
    Collage,
}

/// How many title groups share an attribute value, such as a content type.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupAttributeCountDataPoint {
    pub attribute_value: String,
    pub count: i64,
}

/// Aggregates over every title group of a single catalog, be it an artist, a series or a collage,
/// without any period filter: unlike the torrent stats, these describe the catalog of that entry
/// rather than its upload activity.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TitleGroupStatsResponse {
    pub title_groups_per_release_year: Vec<TitleGroupsPerReleaseYearDataPoint>,
    pub content_types: Vec<TitleGroupAttributeCountDataPoint>,
}
