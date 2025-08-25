use crate::{
    connection_pool::ConnectionPool,
    models::user_application::{
        UserApplication, UserApplicationStatus, UserCreatedUserApplication,
    },
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_user_application(
        &self,
        application: &UserCreatedUserApplication,
    ) -> Result<UserApplication> {
        let gift = sqlx::query_as!(
            UserApplication,
            r#"
                INSERT INTO user_applications (body, referral, email, staff_note, status)
                VALUES ($1, $2, $3, '', 'pending')
                RETURNING id, created_at, body, email, referral, staff_note,
                          status as "status: UserApplicationStatus"
            "#,
            application.body,
            application.referral,
            application.email
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateUserApplication)?;

        Ok(gift)
    }

    pub async fn find_user_applications(
        &self,
        limit: i64,
        page: i64,
        status: Option<UserApplicationStatus>,
    ) -> Result<Vec<UserApplication>> {
        let query = format!(
            r#"
                SELECT id, created_at, body, email, referral, staff_note,
                      status::user_application_status_enum as status
                FROM user_applications ua
                WHERE $1 IS NULL OR ua.status = $1::user_application_status_enum
                ORDER BY created_at DESC
                LIMIT {limit} OFFSET $3
            "#
        );

        let applications = sqlx::query_as::<_, UserApplication>(&query)
            .bind(status)
            .bind(limit)
            .bind((page - 1) * limit)
            .fetch_all(self.borrow())
            .await
            .map_err(Error::CouldNotGetUserApplications)?;

        Ok(applications)
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
                RETURNING id, created_at, body, email, referral, staff_note,
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
