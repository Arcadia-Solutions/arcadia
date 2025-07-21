use sqlx::PgPool;

use crate::{
    Error, Result,
    models::user_application::{UserApplication, UserCreatedUserApplication, UserApplicationStatus},
};

#[derive(Debug)]
pub enum ApplicationFilter {
    All,
    Checked,    // accepted or rejected
    Unchecked,  // pending
    Status(UserApplicationStatus),
}

pub async fn create_user_application(
    pool: &PgPool,
    application: &UserCreatedUserApplication,
) -> Result<UserApplication> {
    let gift = sqlx::query_as!(
        UserApplication,
        r#"
            INSERT INTO user_applications (body, referral, email, staff_note, status, invitation_id)
            VALUES ($1, $2, $3, '', 'pending', NULL)
            RETURNING id, created_at, body, email, referral, staff_note, 
                      status as "status: UserApplicationStatus", invitation_id
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

pub async fn find_user_applications(
    pool: &PgPool,
    limit: Option<i64>,
    offset: Option<i64>,
    filter: ApplicationFilter,
) -> Result<Vec<UserApplication>> {
    let limit = limit.unwrap_or(50); // Default limit of 50
    let offset = offset.unwrap_or(0); // Default offset of 0
    
    let applications = match filter {
        ApplicationFilter::All => {
            sqlx::query_as!(
                UserApplication,
                r#"
                    SELECT id, created_at, body, email, referral, staff_note, 
                           status as "status: UserApplicationStatus", invitation_id
                    FROM user_applications
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(pool)
            .await
        },
        ApplicationFilter::Checked => {
            sqlx::query_as!(
                UserApplication,
                r#"
                    SELECT id, created_at, body, email, referral, staff_note, 
                           status as "status: UserApplicationStatus", invitation_id
                    FROM user_applications
                    WHERE status IN ('accepted', 'rejected')
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(pool)
            .await
        },
        ApplicationFilter::Unchecked => {
            sqlx::query_as!(
                UserApplication,
                r#"
                    SELECT id, created_at, body, email, referral, staff_note, 
                           status as "status: UserApplicationStatus", invitation_id
                    FROM user_applications
                    WHERE status = 'pending'
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(pool)
            .await
        },
        ApplicationFilter::Status(status) => {
            match status {
                UserApplicationStatus::Pending => {
                    sqlx::query_as!(
                        UserApplication,
                        r#"
                            SELECT id, created_at, body, email, referral, staff_note, 
                                   status as "status: UserApplicationStatus", invitation_id
                            FROM user_applications
                            WHERE status = 'pending'
                            ORDER BY created_at DESC
                            LIMIT $1 OFFSET $2
                        "#,
                        limit,
                        offset
                    )
                    .fetch_all(pool)
                    .await
                },
                UserApplicationStatus::Accepted => {
                    sqlx::query_as!(
                        UserApplication,
                        r#"
                            SELECT id, created_at, body, email, referral, staff_note, 
                                   status as "status: UserApplicationStatus", invitation_id
                            FROM user_applications
                            WHERE status = 'accepted'
                            ORDER BY created_at DESC
                            LIMIT $1 OFFSET $2
                        "#,
                        limit,
                        offset
                    )
                    .fetch_all(pool)
                    .await
                },
                UserApplicationStatus::Rejected => {
                    sqlx::query_as!(
                        UserApplication,
                        r#"
                            SELECT id, created_at, body, email, referral, staff_note, 
                                   status as "status: UserApplicationStatus", invitation_id
                            FROM user_applications
                            WHERE status = 'rejected'
                            ORDER BY created_at DESC
                            LIMIT $1 OFFSET $2
                        "#,
                        limit,
                        offset
                    )
                    .fetch_all(pool)
                    .await
                }
            }
        }
    }
    .map_err(Error::CouldNotGetUserApplications)?;

    Ok(applications)
}

pub async fn update_user_application_status(
    pool: &PgPool,
    application_id: i64,
    status: UserApplicationStatus,
    invitation_id: Option<i64>,
) -> Result<UserApplication> {
    let application = match status {
        UserApplicationStatus::Pending => {
            sqlx::query_as!(
                UserApplication,
                r#"
                    UPDATE user_applications 
                    SET status = 'pending', invitation_id = $2
                    WHERE id = $1
                    RETURNING id, created_at, body, email, referral, staff_note, 
                              status as "status: UserApplicationStatus", invitation_id
                "#,
                application_id,
                invitation_id
            )
            .fetch_one(pool)
            .await
        },
        UserApplicationStatus::Accepted => {
            sqlx::query_as!(
                UserApplication,
                r#"
                    UPDATE user_applications 
                    SET status = 'accepted', invitation_id = $2
                    WHERE id = $1
                    RETURNING id, created_at, body, email, referral, staff_note, 
                              status as "status: UserApplicationStatus", invitation_id
                "#,
                application_id,
                invitation_id
            )
            .fetch_one(pool)
            .await
        },
        UserApplicationStatus::Rejected => {
            sqlx::query_as!(
                UserApplication,
                r#"
                    UPDATE user_applications 
                    SET status = 'rejected', invitation_id = $2
                    WHERE id = $1
                    RETURNING id, created_at, body, email, referral, staff_note, 
                              status as "status: UserApplicationStatus", invitation_id
                "#,
                application_id,
                invitation_id
            )
            .fetch_one(pool)
            .await
        }
    }
    .map_err(Error::CouldNotUpdateUserApplication)?;

    Ok(application)
}
