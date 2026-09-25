use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn remove_resolved_and_stale_announce_errors(
    pool: Arc<ConnectionPool>,
    retention_seconds: u64,
) -> Result<u64> {
    let removed_count = pool
        .remove_resolved_and_stale_announce_errors(retention_seconds as i64)
        .await?;
    if removed_count > 0 {
        log::info!(
            "Removed {} resolved or stale announce errors",
            removed_count
        );
    }
    Ok(removed_count)
}
