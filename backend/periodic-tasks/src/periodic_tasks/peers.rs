use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn remove_inactive_peers(
    pool: Arc<ConnectionPool>,
    announce_interval: u32,
    announce_grace_period: u32,
) {
    let removed_peers_amount = pool
        .remove_inactive_peers((announce_interval + announce_grace_period) as f64)
        .await;
    log::info!(
        "Removed {} inactive peers from the database",
        removed_peers_amount.unwrap()
    )
}
