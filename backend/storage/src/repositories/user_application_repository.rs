use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        user_application::{UserApplication, UserApplicationStatus, UserCreatedUserApplication},
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::types::ipnetwork::IpNetwork;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_user_application(
        &self,
        application: &UserCreatedUserApplication,
        from_ip: IpNetwork,
    ) -> Result<UserApplication> {
        let created_application = sqlx::query_as!(
            UserApplication,
            r#"
                INSERT INTO user_applications (body, referral, email, applied_from_ip, staff_note, status)
                VALUES ($1, $2, $3, $4, '', 'pending')
                RETURNING id, created_at, body, email, referral,
                          applied_from_ip as "applied_from_ip: IpNetwork",
                          staff_note, status as "status: UserApplicationStatus"
            "#,
            application.body,
            application.referral,
            application.email,
            from_ip
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateUserApplication)?;

        Ok(created_application)
    }

    pub async fn find_user_applications(
        &self,
        page_size: i64,
        page: i64,
        status: Option<UserApplicationStatus>,
    ) -> Result<PaginatedResults<UserApplication>> {
        let offset = (page - 1) * page_size;

        let total_items: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM user_applications
            WHERE $1::user_application_status_enum IS NULL
               OR status = $1::user_application_status_enum
            "#,
            status.clone() as Option<UserApplicationStatus>
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetUserApplications)?
        .unwrap_or(0);

        let results = sqlx::query_as!(
            UserApplication,
            r#"
            SELECT id, created_at, body, email, referral,
                   applied_from_ip as "applied_from_ip: IpNetwork",
                   staff_note, status as "status: UserApplicationStatus"
            FROM user_applications
            WHERE $1::user_application_status_enum IS NULL
               OR status = $1::user_application_status_enum
            ORDER BY created_at DESC
            OFFSET $2 LIMIT $3
            "#,
            status as Option<UserApplicationStatus>,
            offset,
            page_size
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetUserApplications)?;

        Ok(PaginatedResults {
            results,
            total_items,
            page: page as u32,
            page_size: page_size as u32,
        })
    }

    pub async fn update_user_application_status(
        &self,
        application_id: i64,
        status: UserApplicationStatus,
    ) -> Result<UserApplication> {
        let application = sqlx::query_as::<_, UserApplication>(
            r#"
                UPDATE user_applications
                SET status = $2::user_application_status_enum
                WHERE id = $1
                RETURNING id, created_at, body, email, referral, applied_from_ip, staff_note,
                          status::user_application_status_enum as status
            "#,
        )
        .bind(application_id)
        .bind(status)
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateUserApplication)?;

        Ok(application)
    }
}
