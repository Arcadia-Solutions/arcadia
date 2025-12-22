use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ArcadiaSettings {
    pub user_class_name_on_signup: String,
    pub default_css_sheet_name: String,
    pub open_signups: bool,
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
    pub logo_subtitle: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicArcadiaSettings {
    pub open_signups: bool,
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
    pub logo_subtitle: Option<String>,
}
