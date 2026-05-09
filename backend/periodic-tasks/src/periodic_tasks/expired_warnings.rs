use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn clear_expired_warnings(pool: Arc<ConnectionPool>) -> Result<u64> {
    let cleared_count = pool.clear_expired_warnings().await?;
    if cleared_count > 0 {
        log::info!(
            "Cleared warned status for {} users with expired warnings",
            cleared_count
        );
    }
    Ok(cleared_count)
}
