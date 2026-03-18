use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn clear_expired_warnings(pool: Arc<ConnectionPool>) {
    match pool.clear_expired_warnings().await {
        Ok(cleared_count) => {
            if cleared_count > 0 {
                log::info!(
                    "Cleared warned status for {} users with expired warnings",
                    cleared_count
                );
            }
        }
        Err(e) => {
            log::error!("Error clearing expired warnings: {}", e);
        }
    }
}
