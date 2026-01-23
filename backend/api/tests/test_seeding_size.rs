use arcadia_periodic_tasks::periodic_tasks::seeding_size::update_seeding_size;
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
        "with_test_seeding_size"
    ),
    migrations = "../storage/migrations"
)]
async fn test_seeding_size_calculation(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));

    update_seeding_size(Arc::clone(&pool)).await;

    let pg_pool: &PgPool = (*pool).borrow();

    // User 100: seeds torrent 1 (100MB) + torrent 2 (200MB) = 300MB
    let user_100: (i64,) = sqlx::query_as("SELECT seeding_size FROM users WHERE id = 100")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    assert_eq!(user_100.0, 300000000);

    // User 101: seeds torrent 1 with 3 peers, but size should be counted only once = 100MB
    let user_101: (i64,) = sqlx::query_as("SELECT seeding_size FROM users WHERE id = 101")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    assert_eq!(user_101.0, 100000000);

    // User 102: no peers, should remain at 0
    let user_102: (i64,) = sqlx::query_as("SELECT seeding_size FROM users WHERE id = 102")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    assert_eq!(user_102.0, 0);
}

#[sqlx::test(
    fixtures(
        "with_test_users",
        "with_test_title_group",
        "with_test_edition_group",
        "with_test_torrent",
        "with_test_seeding_size"
    ),
    migrations = "../storage/migrations"
)]
async fn test_seeding_size_reset_when_stopped_seeding(pool: PgPool) {
    let pool = Arc::new(ConnectionPool::with_pg_pool(pool));
    let pg_pool: &PgPool = (*pool).borrow();

    // First update to set initial seeding sizes
    update_seeding_size(Arc::clone(&pool)).await;

    // Verify user 100 has seeding_size set
    let user_100_before: (i64,) = sqlx::query_as("SELECT seeding_size FROM users WHERE id = 100")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    assert_eq!(user_100_before.0, 300000000);

    // Remove all peers for user 100 (simulating stopped seeding)
    sqlx::query("DELETE FROM peers WHERE user_id = 100")
        .execute(pg_pool)
        .await
        .unwrap();

    // Run update again
    update_seeding_size(Arc::clone(&pool)).await;

    // User 100 should now have seeding_size = 0
    let user_100_after: (i64,) = sqlx::query_as("SELECT seeding_size FROM users WHERE id = 100")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    assert_eq!(user_100_after.0, 0);

    // User 101 should still have their seeding_size
    let user_101: (i64,) = sqlx::query_as("SELECT seeding_size FROM users WHERE id = 101")
        .fetch_one(pg_pool)
        .await
        .unwrap();
    assert_eq!(user_101.0, 100000000);
}
