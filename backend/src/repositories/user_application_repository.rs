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
    
    // Build WHERE clause and parameters based on filter
    let (where_clause, _params): (String, Vec<&str>) = match filter {
        ApplicationFilter::All => (String::new(), vec![]),
        ApplicationFilter::Checked => ("WHERE status IN ('accepted', 'rejected')".to_string(), vec![]),
        ApplicationFilter::Unchecked => ("WHERE status = 'pending'".to_string(), vec![]),
        ApplicationFilter::Status(status) => {
            let status_str = match status {
                UserApplicationStatus::Pending => "pending",
                UserApplicationStatus::Accepted => "accepted",
                UserApplicationStatus::Rejected => "rejected",
            };
            (format!("WHERE status = '{status_str}'"), vec![])
        }
    };
    
    // Construct the full query
    let query = format!(
        r#"
            SELECT id, created_at, body, email, referral, staff_note,     
                   status as "status: UserApplicationStatus", invitation_id
            FROM user_applications
            {where_clause}
            ORDER BY created_at DESC
            LIMIT {limit} OFFSET {offset}
        "#
    );
    
    let applications = sqlx::query_as::<_, UserApplication>(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(Error::CouldNotGetUserApplications)?;

    Ok(applications)
}

pub async fn update_user_application_status(
    pool: &PgPool,
    application_id: i64,
    status: UserApplicationStatus,
    invitation_id: Option<i64>,
) -> Result<UserApplication> {
    let status_str = match status {
        UserApplicationStatus::Pending => "pending",
        UserApplicationStatus::Accepted => "accepted",
        UserApplicationStatus::Rejected => "rejected",
    };
    
    let query = format!(
        r#"
            UPDATE user_applications
            SET status = '{status_str}', invitation_id = $2
            WHERE id = $1
            RETURNING id, created_at, body, email, referral, staff_note,
                      status as "status: UserApplicationStatus", invitation_id
        "#
    );
    
    let application = sqlx::query_as::<_, UserApplication>(&query)
        .bind(application_id)
        .bind(invitation_id)
        .fetch_one(pool)
        .await
        .map_err(Error::CouldNotUpdateUserApplication)?;

    Ok(application)
}
