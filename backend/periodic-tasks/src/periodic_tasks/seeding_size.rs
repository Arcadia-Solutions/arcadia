use arcadia_storage::connection_pool::ConnectionPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn update_seeding_size(pool: Arc<ConnectionPool>) {
    println!("updating seeding size");
    match update_seeding_size_inner(&pool).await {
        Ok(updated_count) => {
            log::info!("Updated seeding_size for {} users", updated_count);
        }
        Err(e) => {
            log::error!("Error updating seeding_size: {}", e);
        }
    }
}

async fn update_seeding_size_inner(pool: &ConnectionPool) -> Result<u64, sqlx::Error> {
    // Update seeding_size for all users who are either:
    // - Currently seeding (have entries in peers table as seeder)
    // - Have a non-zero seeding_size that needs to be reset (stopped seeding)
    // Only updates rows where the value actually changes for efficiency
    let result = sqlx::query!(
        r#"
        WITH unique_seeding_torrents AS (
            SELECT DISTINCT user_id, torrent_id
            FROM peers
            WHERE seeder = true
        ),
        seeding_totals AS (
            SELECT ust.user_id, SUM(t.size) as total_size
            FROM unique_seeding_torrents ust
            JOIN torrents t ON ust.torrent_id = t.id
            GROUP BY ust.user_id
        ),
        users_to_update AS (
            SELECT u.id, COALESCE(st.total_size, 0) as new_seeding_size
            FROM users u
            LEFT JOIN seeding_totals st ON st.user_id = u.id
            WHERE st.user_id IS NOT NULL OR u.seeding_size > 0
        )
        UPDATE users u
        SET seeding_size = utu.new_seeding_size
        FROM users_to_update utu
        WHERE u.id = utu.id
          AND u.seeding_size != utu.new_seeding_size
        "#
    )
    .execute(pool.borrow())
    .await?;

    Ok(result.rows_affected())
}
