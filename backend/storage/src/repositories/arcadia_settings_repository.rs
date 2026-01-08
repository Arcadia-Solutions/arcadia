use crate::{connection_pool::ConnectionPool, models::arcadia_settings::ArcadiaSettings};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn get_arcadia_settings(&self) -> Result<ArcadiaSettings> {
        let settings = sqlx::query_as!(
            ArcadiaSettings,
            r#"
                SELECT *
                FROM arcadia_settings
                LIMIT 1
            "#,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindArcadiaSettings)?;

        Ok(settings)
    }

    pub async fn update_arcadia_settings(
        &self,
        settings: &ArcadiaSettings,
    ) -> Result<ArcadiaSettings> {
        let updated_settings = sqlx::query_as!(
            ArcadiaSettings,
            r#"
                UPDATE arcadia_settings
                SET user_class_name_on_signup = $1,
                    default_css_sheet_name = $2,
                    open_signups = $3,
                    global_upload_factor = $4,
                    global_download_factor = $5,
                    logo_subtitle = $6,
                    approved_image_hosts = $7,
                    upload_page_top_text = $8
                RETURNING *
            "#,
            settings.user_class_name_on_signup,
            settings.default_css_sheet_name,
            settings.open_signups,
            settings.global_upload_factor,
            settings.global_download_factor,
            settings.logo_subtitle,
            &settings.approved_image_hosts,
            settings.upload_page_top_text
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateArcadiaSettings)?;

        Ok(updated_settings)
    }
}
