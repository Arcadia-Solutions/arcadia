use arcadia_storage::connection_pool::ConnectionPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn update_bonus_points(
    pool: Arc<ConnectionPool>,
    formula_sql: String,
    increment_seconds: u64,
) {
    match update_bonus_points_inner(&pool, &formula_sql, increment_seconds).await {
        Ok(updated_count) => {
            log::info!("Updated bonus points for {} users", updated_count);
        }
        Err(e) => {
            log::error!("Error updating bonus points: {}", e);
        }
    }
}

async fn update_bonus_points_inner(
    pool: &ConnectionPool,
    formula_sql: &str,
    increment_seconds: u64,
) -> Result<u64, sqlx::Error> {
    let query = format!(
        r#"
        WITH activity_bonus AS (
            SELECT
                ta.id AS activity_id,
                ta.user_id,
                ROUND({formula})::bigint AS bonus
            FROM torrent_activities ta
            INNER JOIN torrents t ON ta.torrent_id = t.id
            INNER JOIN peers p ON p.torrent_id = t.id AND p.user_id = ta.user_id
            WHERE p.seeder = true AND p.active = true
        ),
        update_activities AS (
            UPDATE torrent_activities ta
            SET bonus_points = ta.bonus_points + ab.bonus
            FROM activity_bonus ab
            WHERE ta.id = ab.activity_id AND ab.bonus > 0
        )
        UPDATE users u
        SET bonus_points = u.bonus_points + user_bonus.total_bonus
        FROM (
            SELECT user_id, SUM(bonus) AS total_bonus
            FROM activity_bonus
            WHERE bonus > 0
            GROUP BY user_id
        ) AS user_bonus
        WHERE u.id = user_bonus.user_id
        "#,
        formula = formula_sql
    );

    let result = sqlx::query(&query)
        .bind(increment_seconds as i64)
        .execute(pool.borrow())
        .await?;

    Ok(result.rows_affected())
}
