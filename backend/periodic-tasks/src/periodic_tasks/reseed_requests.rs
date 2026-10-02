use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn remove_resolved_reseed_requests(pool: Arc<ConnectionPool>) -> Result<u64> {
    let removed_count = pool.remove_resolved_reseed_requests().await?;
    if removed_count > 0 {
        log::info!("Removed {} resolved reseed requests", removed_count);
    }
    Ok(removed_count)
}
