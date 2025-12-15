use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::store::Store;

pub async fn run_periodic_tasks(store: Arc<Store>) -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    // Task 1: Update torrent seeders/leechers
    let pool = Arc::clone(&store.pool);
    let schedule = store.env.tasks.update_seeders_leechers.clone();
    sched
        .add(Job::new_async(schedule.as_str(), move |_, _| {
            let pool = Arc::clone(&pool);
            Box::pin(async move {
                match pool.update_seeders_leechers().await {
                    Ok(count) => log::info!("Updated seeders/leechers for {count} torrents"),
                    Err(e) => log::error!("Error updating seeders/leechers: {e}"),
                }
            })
        })?)
        .await?;
    log::info!(
        "Scheduled: update_seeders_leechers ({})",
        store.env.tasks.update_seeders_leechers
    );

    // Task 2: Remove inactive peers
    let pool = Arc::clone(&store.pool);
    let schedule = store.env.tasks.remove_inactive_peers.clone();
    let timeout = (store.env.tracker.announce_interval
        + store.env.tracker.announce_interval_grace_period) as f64;
    sched
        .add(Job::new_async(schedule.as_str(), move |_, _| {
            let pool = Arc::clone(&pool);
            Box::pin(async move {
                match pool.remove_inactive_peers(timeout).await {
                    Ok(count) => log::info!("Removed {count} inactive peers"),
                    Err(e) => log::error!("Error removing inactive peers: {e}"),
                }
            })
        })?)
        .await?;
    log::info!(
        "Scheduled: remove_inactive_peers ({}) - timeout: {timeout}s",
        store.env.tasks.remove_inactive_peers
    );

    // Task 3: Expire user warnings
    let pool = Arc::clone(&store.pool);
    let schedule = store.env.tasks.expire_warnings.clone();
    sched
        .add(Job::new_async(schedule.as_str(), move |_, _| {
            let pool = Arc::clone(&pool);
            Box::pin(async move {
                match pool.expire_warnings().await {
                    Ok(count) if count > 0 => log::info!("Expired warnings for {count} users"),
                    Ok(_) => {}
                    Err(e) => log::error!("Error expiring warnings: {e}"),
                }
            })
        })?)
        .await?;
    log::info!(
        "Scheduled: expire_warnings ({})",
        store.env.tasks.expire_warnings
    );

    // Task 4: Disable inactive users
    let pool = Arc::clone(&store.pool);
    let schedule = store.env.tasks.disable_inactive_users.clone();
    let days = store.env.tasks.inactive_user_days;
    sched
        .add(Job::new_async(schedule.as_str(), move |_, _| {
            let pool = Arc::clone(&pool);
            Box::pin(async move {
                match pool.disable_inactive_users(days).await {
                    Ok(count) if count > 0 => {
                        log::info!("Disabled {count} inactive users (>{days} days)")
                    }
                    Ok(_) => {}
                    Err(e) => log::error!("Error disabling inactive users: {e}"),
                }
            })
        })?)
        .await?;
    log::info!(
        "Scheduled: disable_inactive_users ({}) - threshold: {days} days",
        store.env.tasks.disable_inactive_users
    );

    // Task 5: Update user seeding sizes
    let pool = Arc::clone(&store.pool);
    let schedule = store.env.tasks.update_seeding_size.clone();
    sched
        .add(Job::new_async(schedule.as_str(), move |_, _| {
            let pool = Arc::clone(&pool);
            Box::pin(async move {
                match pool.update_all_seeding_sizes().await {
                    Ok(count) => log::info!("Updated seeding_size for {count} users"),
                    Err(e) => log::error!("Error updating seeding_size: {e}"),
                }
            })
        })?)
        .await?;
    log::info!(
        "Scheduled: update_seeding_size ({})",
        store.env.tasks.update_seeding_size
    );

    log::info!("All periodic tasks scheduled. Starting scheduler...");
    sched.start().await?;

    // Keep running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
