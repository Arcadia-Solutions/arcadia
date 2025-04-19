use crate::{
    Error, Result,
    models::master_group::{MasterGroup, UserCreatedMasterGroup},
};
use sqlx::PgPool;

pub async fn create_master_group(
    pool: &PgPool,
    master_group_form: &UserCreatedMasterGroup,
    current_user_id: i64,
) -> Result<MasterGroup> {
    let created_master_group = sqlx::query_as!(
        MasterGroup,
        r#"
            INSERT INTO master_groups (name,created_by_id)
            VALUES ($1, $2)
            RETURNING *
        "#,
        master_group_form.name,
        current_user_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateMasterGroup)?;

    Ok(created_master_group)
}
