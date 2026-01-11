use arcadia_storage::connection_pool::ConnectionPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn update_seedtime(pool: Arc<ConnectionPool>, increment_seconds: u64) {
    match update_seedtime_inner(&pool, increment_seconds).await {
        Ok(updated_count) => {
            log::info!(
                "Updated seedtime for {} torrent activities (+{}s)",
                updated_count,
                increment_seconds
            );
        }
        Err(e) => {
            log::error!("Error updating seedtime: {}", e);
        }
    }
}

async fn update_seedtime_inner(
    pool: &ConnectionPool,
    increment_seconds: u64,
) -> Result<u64, sqlx::Error> {
    // Update total_seed_time for all torrent_activities where the user
    // has an active seeding peer for that torrent
    let result = sqlx::query!(
        r#"
        UPDATE torrent_activities ta
        SET total_seed_time = total_seed_time + $1
        WHERE EXISTS (
            SELECT 1 FROM peers p
            WHERE p.torrent_id = ta.torrent_id
              AND p.user_id = ta.user_id
              AND p.seeder = true
              AND p.active = true
        )
        "#,
        increment_seconds as i64
    )
    .execute(pool.borrow())
    .await?;

    Ok(result.rows_affected())
}
