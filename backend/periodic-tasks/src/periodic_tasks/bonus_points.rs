use arcadia_storage::connection_pool::ConnectionPool;
use sqlx::PgPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn update_seedtime_and_bonus_points(
    pool: Arc<ConnectionPool>,
    increment_seconds: u64,
    formula_sql: String,
) {
    match update_seedtime_and_bonus_points_inner(&pool, increment_seconds, &formula_sql).await {
        Ok((seedtime_count, bonus_count)) => {
            log::info!(
                "Updated seedtime for {} torrent activities (+{}s), bonus points for {} users",
                seedtime_count,
                increment_seconds,
                bonus_count
            );
        }
        Err(e) => {
            log::error!("Error updating seedtime and bonus points: {}", e);
        }
    }
}

async fn update_seedtime_and_bonus_points_inner(
    pool: &ConnectionPool,
    increment_seconds: u64,
    formula_sql: &str,
) -> Result<(u64, u64), sqlx::Error> {
    let mut transaction = <ConnectionPool as Borrow<PgPool>>::borrow(pool)
        .begin()
        .await?;

    let seedtime_result = sqlx::query!(
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
    .execute(&mut *transaction)
    .await?;

    let bonus_query = format!(
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

    let bonus_result = sqlx::query(&bonus_query).execute(&mut *transaction).await?;

    transaction.commit().await?;

    Ok((
        seedtime_result.rows_affected(),
        bonus_result.rows_affected(),
    ))
}
