use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "entity_role_enum")]
pub enum EntityRole {
    #[serde(rename = "producer")]
    #[sqlx(rename = "producer")]
    Producer,
    #[serde(rename = "developer")]
    #[sqlx(rename = "developer")]
    Developer,
    #[serde(rename = "designer")]
    #[sqlx(rename = "designer")]
    Designer,
    #[serde(rename = "label")]
    #[sqlx(rename = "label")]
    Label,
    #[serde(rename = "network")]
    #[sqlx(rename = "network")]
    Network,
}
#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct Entity {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub description: String,
    pub pictures: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct AffiliatedEntity {
    pub id: i64,
    pub title_group_id: i64,
    pub entity_id: i64,
    pub created_by_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub roles: Vec<EntityRole>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AffiliatedEntityHierarchy {
    pub id: i64,
    pub title_group_id: i64,
    pub entity_id: i64,
    pub created_by_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub roles: Vec<EntityRole>,
    pub entity: Entity,
}
