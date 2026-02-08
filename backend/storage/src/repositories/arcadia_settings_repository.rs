use crate::{
    connection_pool::ConnectionPool,
    models::arcadia_settings::{ArcadiaSettings, SnatchedTorrentBonusPointsTransferredTo},
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn get_arcadia_settings(&self) -> Result<ArcadiaSettings> {
        let settings = sqlx::query_as!(
            ArcadiaSettings,
            r#"
                SELECT
                    user_class_name_on_signup,
                    default_css_sheet_name,
                    open_signups,
                    global_upload_factor,
                    global_download_factor,
                    logo_subtitle,
                    approved_image_hosts,
                    upload_page_top_text,
                    automated_message_on_signup,
                    automated_message_on_signup_sender_id,
                    automated_message_on_signup_locked,
                    automated_message_on_signup_conversation_name,
                    bonus_points_given_on_upload,
                    allow_uploader_set_torrent_bonus_points_cost,
                    default_torrent_bonus_points_cost,
                    torrent_bonus_points_cost_min,
                    torrent_bonus_points_cost_max,
                    shop_upload_base_price_per_gb,
                    shop_upload_discount_tiers,
                    shop_freeleech_token_base_price,
                    shop_freeleech_token_discount_tiers,
                    bonus_points_alias,
                    bonus_points_decimal_places,
                    torrent_max_release_date_allowed,
                    snatched_torrent_bonus_points_transferred_to as "snatched_torrent_bonus_points_transferred_to: _"
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
                    upload_page_top_text = $8,
                    automated_message_on_signup = $9,
                    automated_message_on_signup_sender_id = $10,
                    automated_message_on_signup_locked = $11,
                    automated_message_on_signup_conversation_name = $12,
                    bonus_points_given_on_upload = $13,
                    allow_uploader_set_torrent_bonus_points_cost = $14,
                    default_torrent_bonus_points_cost = $15,
                    torrent_bonus_points_cost_min = $16,
                    torrent_bonus_points_cost_max = $17,
                    shop_upload_base_price_per_gb = $18,
                    shop_upload_discount_tiers = $19,
                    shop_freeleech_token_base_price = $20,
                    shop_freeleech_token_discount_tiers = $21,
                    bonus_points_alias = $22,
                    bonus_points_decimal_places = $23,
                    torrent_max_release_date_allowed = $24,
                    snatched_torrent_bonus_points_transferred_to = $25
                RETURNING
                    user_class_name_on_signup,
                    default_css_sheet_name,
                    open_signups,
                    global_upload_factor,
                    global_download_factor,
                    logo_subtitle,
                    approved_image_hosts,
                    upload_page_top_text,
                    automated_message_on_signup,
                    automated_message_on_signup_sender_id,
                    automated_message_on_signup_locked,
                    automated_message_on_signup_conversation_name,
                    bonus_points_given_on_upload,
                    allow_uploader_set_torrent_bonus_points_cost,
                    default_torrent_bonus_points_cost,
                    torrent_bonus_points_cost_min,
                    torrent_bonus_points_cost_max,
                    shop_upload_base_price_per_gb,
                    shop_upload_discount_tiers,
                    shop_freeleech_token_base_price,
                    shop_freeleech_token_discount_tiers,
                    bonus_points_alias,
                    bonus_points_decimal_places,
                    torrent_max_release_date_allowed,
                    snatched_torrent_bonus_points_transferred_to as "snatched_torrent_bonus_points_transferred_to: _"
            "#,
            settings.user_class_name_on_signup,
            settings.default_css_sheet_name,
            settings.open_signups,
            settings.global_upload_factor,
            settings.global_download_factor,
            settings.logo_subtitle,
            &settings.approved_image_hosts,
            settings.upload_page_top_text,
            settings.automated_message_on_signup,
            settings.automated_message_on_signup_sender_id,
            settings.automated_message_on_signup_locked,
            settings.automated_message_on_signup_conversation_name,
            settings.bonus_points_given_on_upload,
            settings.allow_uploader_set_torrent_bonus_points_cost,
            settings.default_torrent_bonus_points_cost,
            settings.torrent_bonus_points_cost_min,
            settings.torrent_bonus_points_cost_max,
            settings.shop_upload_base_price_per_gb,
            settings.shop_upload_discount_tiers,
            settings.shop_freeleech_token_base_price,
            settings.shop_freeleech_token_discount_tiers,
            &settings.bonus_points_alias,
            settings.bonus_points_decimal_places,
            settings.torrent_max_release_date_allowed,
            settings
                .snatched_torrent_bonus_points_transferred_to
                .clone() as Option<SnatchedTorrentBonusPointsTransferredTo>
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateArcadiaSettings)?;

        Ok(updated_settings)
    }
}
