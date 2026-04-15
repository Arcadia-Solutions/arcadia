// Note: the peers at the torrent level (torrents.seeders, torrents.leechers,
// torrents.times_completed) are updated by the tracker itself directly.
// This module only aggregates those torrent-level values up to the artist level.

use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub async fn update_artist_peer_stats(pool: Arc<ConnectionPool>) {
    match pool.update_artist_peer_stats().await {
        Ok(updated_count) => {
            log::info!("Updated peer stats for {} artists", updated_count);
        }
        Err(error) => {
            log::error!("Error updating artist peer stats: {}", error);
        }
    }
}
