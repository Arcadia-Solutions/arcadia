use sqlx::PgPool;

use crate::{
    Error, Result,
    models::user_application::{UserApplication, UserCreatedUserApplication},
};

pub async fn create_user_application(
    pool: &PgPool,
    application: &UserCreatedUserApplication,
) -> Result<UserApplication> {
    let gift = sqlx::query_as!(
        UserApplication,
        r#"
            INSERT INTO user_applications (body, referral, email, staff_note)
            VALUES ($1, $2, $3, '')
            RETURNING *
        "#,
        application.body,
        application.referral,
        application.email
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateUserApplication)?;

    Ok(gift)
}

pub async fn get_all_user_applications(
    pool: &PgPool,
) -> Result<Vec<UserApplication>> {
    let applications = sqlx::query_as!(
        UserApplication,
        r#"
            SELECT * FROM user_applications
            ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(Error::CouldNotGetUserApplications)?;

    Ok(applications)
}
