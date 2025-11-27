use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResults<T> {
    pub results: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total_items: i64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Display)]
pub enum OrderByDirection {
    #[serde(rename = "asc")]
    #[strum(serialize = "asc")]
    Asc,
    #[serde(rename = "desc")]
    #[strum(serialize = "desc")]
    Desc,
}
