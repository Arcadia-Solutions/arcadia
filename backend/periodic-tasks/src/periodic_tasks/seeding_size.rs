use arcadia_storage::connection_pool::ConnectionPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn update_user_torrent_stats(pool: Arc<ConnectionPool>) {
    println!("updating user torrent stats");
    match update_user_torrent_stats_inner(&pool).await {
        Ok(updated_count) => {
            log::info!("Updated torrent stats for {} users", updated_count);
        }
        Err(e) => {
            log::error!("Error updating torrent stats: {}", e);
        }
    }
}

async fn update_user_torrent_stats_inner(pool: &ConnectionPool) -> Result<u64, sqlx::Error> {
    // Update seeding_size, seeding, leeching, and snatched for all users who either:
    // - Have active peers in the peers table
    // - Have completed torrent activities (snatched)
    // - Have non-zero values that need to be reset
    // Only updates rows where at least one value actually changes for efficiency
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
        peer_counts AS (
            SELECT
                user_id,
                COUNT(DISTINCT CASE WHEN seeder = true THEN torrent_id END)::INTEGER as seeding_count,
                COUNT(DISTINCT CASE WHEN seeder = false THEN torrent_id END)::INTEGER as leeching_count
            FROM peers
            GROUP BY user_id
        ),
        snatched_counts AS (
            SELECT user_id, COUNT(*)::INTEGER as snatched_count
            FROM torrent_activities
            WHERE completed_at IS NOT NULL
            GROUP BY user_id
        ),
        average_seed_times AS (
            SELECT user_id,
                   CASE WHEN COUNT(*) > 0
                        THEN SUM(total_seed_time) / COUNT(*)
                        ELSE 0
                   END as average_seed_time
            FROM torrent_activities
            WHERE total_seed_time > 0
            GROUP BY user_id
        ),
        users_to_update AS (
            SELECT
                u.id,
                COALESCE(st.total_size, 0) as new_seeding_size,
                COALESCE(pc.seeding_count, 0) as new_seeding,
                COALESCE(pc.leeching_count, 0) as new_leeching,
                COALESCE(sc.snatched_count, 0) as new_snatched,
                COALESCE(ast.average_seed_time, 0) as new_average_seeding_time
            FROM users u
            LEFT JOIN seeding_totals st ON st.user_id = u.id
            LEFT JOIN peer_counts pc ON pc.user_id = u.id
            LEFT JOIN snatched_counts sc ON sc.user_id = u.id
            LEFT JOIN average_seed_times ast ON ast.user_id = u.id
            WHERE st.user_id IS NOT NULL
               OR pc.user_id IS NOT NULL
               OR sc.user_id IS NOT NULL
               OR ast.user_id IS NOT NULL
               OR u.seeding_size > 0
               OR u.seeding > 0
               OR u.leeching > 0
               OR u.snatched > 0
               OR u.average_seeding_time > 0
        )
        UPDATE users u
        SET seeding_size = utu.new_seeding_size,
            seeding = utu.new_seeding,
            leeching = utu.new_leeching,
            snatched = utu.new_snatched,
            average_seeding_time = utu.new_average_seeding_time
        FROM users_to_update utu
        WHERE u.id = utu.id
          AND (u.seeding_size != utu.new_seeding_size
            OR u.seeding != utu.new_seeding
            OR u.leeching != utu.new_leeching
            OR u.snatched != utu.new_snatched
            OR u.average_seeding_time != utu.new_average_seeding_time)
        "#
    )
    .execute(pool.borrow())
    .await?;

    Ok(result.rows_affected())
}
