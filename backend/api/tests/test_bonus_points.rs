use arcadia_periodic_tasks::{
    env::formula_to_sql, periodic_tasks::bonus_points::update_seedtime_and_bonus_points,
};
use arcadia_storage::connection_pool::ConnectionPool;
use sqlx::PgPool;
use std::borrow::Borrow;
use std::sync::Arc;

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_bonus_points"
    ),
    migrations = "../storage/migrations"
)]
async fn test_bonus_points_calculation(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    // Formula using all 3 variables: seedtime / 100 + size / 100000000 - seeders
    // Torrent 1: size=100MB, seeders=2 | Torrent 2: size=200MB, seeders=1
    // User 100: t1(200/100 + 1 - 2 = 1) + t2(300/100 + 2 - 1 = 4) = 5 total
    // User 101: t1(400/100 + 1 - 2 = 3) + t2(500/100 + 2 - 1 = 6) = 9 total
    let formula_sql =
        formula_to_sql("seedtime / 100 + size / 100000000 - seeders", "t.seeders").unwrap();
    update_seedtime_and_bonus_points(Arc::clone(&pool), 0, formula_sql).await;

    let pg_pool: &PgPool = (*pool).borrow();

    // Verify torrent_activities bonus_points
    let activities: Vec<(i32, i32, i64)> = sqlx::query_as(
        "SELECT torrent_id, user_id, bonus_points FROM torrent_activities ORDER BY user_id, torrent_id",
    )
    .fetch_all(pg_pool)
    .await
    .unwrap();

    assert_eq!(activities.len(), 4);
    assert_eq!(activities[0], (1, 100, 1)); // user 100, torrent 1: 200/100 + 1 - 2 = 1
    assert_eq!(activities[1], (2, 100, 4)); // user 100, torrent 2: 300/100 + 2 - 1 = 4
    assert_eq!(activities[2], (1, 101, 3)); // user 101, torrent 1: 400/100 + 1 - 2 = 3
    assert_eq!(activities[3], (2, 101, 6)); // user 101, torrent 2: 500/100 + 2 - 1 = 6

    // Verify users bonus_points
    let user_100_bonus: (i64,) = sqlx::query_as("SELECT bonus_points FROM users WHERE id = 100")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    let user_101_bonus: (i64,) = sqlx::query_as("SELECT bonus_points FROM users WHERE id = 101")
        .fetch_one(pg_pool)
        .await
        .unwrap();

    assert_eq!(user_100_bonus.0, 5); // 1 + 4 = 5
    assert_eq!(user_101_bonus.0, 9); // 3 + 6 = 9

    // --- Test with LOG formula ---
    // Reset bonus_points for second test
    sqlx::query("UPDATE users SET bonus_points = 0 WHERE id IN (100, 101)")
        .execute(pg_pool)
        .await
        .unwrap();
    sqlx::query("UPDATE torrent_activities SET bonus_points = 0 WHERE user_id IN (100, 101)")
        .execute(pg_pool)
        .await
        .unwrap();

    // Formula with LOG using all 3 variables: LN(seedtime * size / 1000000000) + seeders
    // User 100: t1(LN(20)+2 ≈ 5) + t2(LN(60)+1 ≈ 5) = 10 total
    // User 101: t1(LN(40)+2 ≈ 6) + t2(LN(100)+1 ≈ 6) = 12 total
    let log_formula_sql =
        formula_to_sql("LN(seedtime * size / 1000000000) + seeders", "t.seeders").unwrap();
    update_seedtime_and_bonus_points(Arc::clone(&pool), 0, log_formula_sql).await;

    let log_activities: Vec<(i32, i32, i64)> = sqlx::query_as(
        "SELECT torrent_id, user_id, bonus_points FROM torrent_activities ORDER BY user_id, torrent_id",
    )
    .fetch_all(pg_pool)
    .await
    .unwrap();

    assert_eq!(log_activities.len(), 4);
    assert_eq!(log_activities[0], (1, 100, 5)); // LN(200*100M/1B) + 2 = LN(20) + 2 ≈ 5
    assert_eq!(log_activities[1], (2, 100, 5)); // LN(300*200M/1B) + 1 = LN(60) + 1 ≈ 5
    assert_eq!(log_activities[2], (1, 101, 6)); // LN(400*100M/1B) + 2 = LN(40) + 2 ≈ 6
    assert_eq!(log_activities[3], (2, 101, 6)); // LN(500*200M/1B) + 1 = LN(100) + 1 ≈ 6

    let user_100_log: (i64,) = sqlx::query_as("SELECT bonus_points FROM users WHERE id = 100")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    let user_101_log: (i64,) = sqlx::query_as("SELECT bonus_points FROM users WHERE id = 101")
        .fetch_one(pg_pool)
        .await
        .unwrap();

    assert_eq!(user_100_log.0, 10); // 5 + 5 = 10
    assert_eq!(user_101_log.0, 12); // 6 + 6 = 12
}
