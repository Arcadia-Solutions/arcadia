use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        donation::{
            Donation, DonationSettings, DonationStats, EditedDonation, EditedDonationSettings,
            UserCreatedDonation,
        },
    },
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn find_donations(
        &self,
        page_size: i64,
        page: i64,
    ) -> Result<PaginatedResults<Donation>> {
        let offset = (page - 1) * page_size;

        let total_items: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM donations
            "#
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetDonations)?
        .unwrap_or(0);

        let results = sqlx::query_as!(
            Donation,
            r#"
            SELECT id, created_at, amount, currency, donor_name, user_id, note, created_by_id
            FROM donations
            ORDER BY created_at DESC
            OFFSET $1 LIMIT $2
            "#,
            offset,
            page_size
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetDonations)?;

        Ok(PaginatedResults {
            results,
            total_items,
            page: page as u32,
            page_size: page_size as u32,
        })
    }

    pub async fn find_donation_by_id(&self, id: i64) -> Result<Donation> {
        sqlx::query_as!(
            Donation,
            r#"
            SELECT id, created_at, amount, currency, donor_name, user_id, note, created_by_id
            FROM donations
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::DonationNotFound(id))
    }

    pub async fn create_donation(
        &self,
        donation: &UserCreatedDonation,
        created_by_id: i32,
    ) -> Result<Donation> {
        sqlx::query_as!(
            Donation,
            r#"
            INSERT INTO donations (amount, currency, donor_name, user_id, note, created_by_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, created_at, amount, currency, donor_name, user_id, note, created_by_id
            "#,
            donation.amount,
            donation.currency,
            donation.donor_name,
            donation.user_id,
            donation.note,
            created_by_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateDonation)
    }

    pub async fn update_donation(&self, id: i64, donation: &EditedDonation) -> Result<Donation> {
        sqlx::query_as!(
            Donation,
            r#"
            UPDATE donations
            SET amount = $2, currency = $3, donor_name = $4, user_id = $5, note = $6
            WHERE id = $1
            RETURNING id, created_at, amount, currency, donor_name, user_id, note, created_by_id
            "#,
            id,
            donation.amount,
            donation.currency,
            donation.donor_name,
            donation.user_id,
            donation.note
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::DonationNotFound(id))
    }

    pub async fn delete_donation(&self, id: i64) -> Result<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM donations
            WHERE id = $1
            "#,
            id
        )
        .execute(self.borrow())
        .await
        .map_err(|_| Error::DonationNotFound(id))?;

        if result.rows_affected() == 0 {
            return Err(Error::DonationNotFound(id));
        }

        Ok(())
    }

    pub async fn get_donation_settings(&self) -> Result<DonationSettings> {
        sqlx::query_as!(
            DonationSettings,
            r#"
            SELECT donation_goal, donation_goal_period
            FROM donation_settings
            WHERE id = 1
            "#
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetDonationSettings)
    }

    pub async fn update_donation_settings(
        &self,
        settings: &EditedDonationSettings,
    ) -> Result<DonationSettings> {
        sqlx::query_as!(
            DonationSettings,
            r#"
            UPDATE donation_settings
            SET donation_goal = $1, donation_goal_period = $2
            WHERE id = 1
            RETURNING donation_goal, donation_goal_period
            "#,
            settings.donation_goal,
            settings.donation_goal_period
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateDonationSettings)
    }

    pub async fn get_donation_stats(&self) -> Result<DonationStats> {
        let settings = self.get_donation_settings().await?;

        let current_total: f64 = match settings.donation_goal_period.as_str() {
            "monthly" => {
                sqlx::query_scalar!(
                    r#"
                    SELECT COALESCE(SUM(amount), 0) as "total!"
                    FROM donations
                    WHERE created_at >= DATE_TRUNC('month', CURRENT_DATE)
                      AND created_at < DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month'
                    "#
                )
                .fetch_one(self.borrow())
                .await
                .map_err(Error::CouldNotGetDonationStats)?
            }
            "yearly" => {
                sqlx::query_scalar!(
                    r#"
                    SELECT COALESCE(SUM(amount), 0) as "total!"
                    FROM donations
                    WHERE created_at >= DATE_TRUNC('year', CURRENT_DATE)
                      AND created_at < DATE_TRUNC('year', CURRENT_DATE) + INTERVAL '1 year'
                    "#
                )
                .fetch_one(self.borrow())
                .await
                .map_err(Error::CouldNotGetDonationStats)?
            }
            _ => {
                sqlx::query_scalar!(
                    r#"
                    SELECT COALESCE(SUM(amount), 0) as "total!"
                    FROM donations
                    WHERE created_at >= DATE_TRUNC('month', CURRENT_DATE)
                      AND created_at < DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month'
                    "#
                )
                .fetch_one(self.borrow())
                .await
                .map_err(Error::CouldNotGetDonationStats)?
            }
        };

        Ok(DonationStats {
            current_total,
            goal: settings.donation_goal,
            period: settings.donation_goal_period,
        })
    }
}
