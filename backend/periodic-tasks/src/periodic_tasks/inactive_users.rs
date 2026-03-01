use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn ban_inactive_users(pool: Arc<ConnectionPool>) {
    match ban_inactive_users_inner(&pool).await {
        Ok(Some(banned_count)) => {
            log::info!("Banned {} inactive users", banned_count);
        }
        Ok(None) => {
            log::debug!("Inactive user ban is disabled, skipping");
        }
        Err(e) => {
            log::error!("Error banning inactive users: {}", e);
        }
    }
}

async fn ban_inactive_users_inner(
    pool: &ConnectionPool,
) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    let settings = pool.get_arcadia_settings().await?;

    let Some(inactive_days) = settings.inactive_user_ban_after_days else {
        return Ok(None);
    };

    let banned_count = pool.ban_inactive_users(inactive_days).await?;

    Ok(Some(banned_count))
}
