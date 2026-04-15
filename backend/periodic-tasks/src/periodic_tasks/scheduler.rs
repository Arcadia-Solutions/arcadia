use std::sync::Arc;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::store::Store;

use super::bonus_points::update_seedtime_and_bonus_points;
use super::expired_warnings::clear_expired_warnings;
use super::inactive_users::ban_inactive_users;
use super::materialized_views::refresh_title_group_hierarchy_lite;
use super::peers::update_artist_peer_stats;
use super::seeding_size::update_user_torrent_stats;
use super::user_classes::process_user_class_changes;

pub async fn run_periodic_tasks(
    store: Arc<Store>,
) -> Result<JobScheduler, Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    // User class promotion/demotion task
    let pool_1 = Arc::clone(&store.pool);
    let user_class_job = Job::new_repeated_async(
        Duration::from_secs(store.env.periodic_tasks.user_class_changes_seconds),
        move |_uuid, _l| Box::pin(process_user_class_changes(Arc::clone(&pool_1))),
    )?;
    sched.add(user_class_job).await?;

    // Seedtime and bonus points update task (combined to avoid deadlocks on torrent_activities)
    let pool_2 = Arc::clone(&store.pool);
    let seedtime_and_bonus_points_interval_secs = store
        .env
        .periodic_tasks
        .seedtime_and_bonus_points_update_seconds;
    let bonus_formula_sql = store.env.periodic_tasks.bonus_points_formula.clone();
    let seedtime_and_bonus_job = Job::new_repeated_async(
        Duration::from_secs(seedtime_and_bonus_points_interval_secs),
        move |_uuid, _l| {
            let formula_sql = bonus_formula_sql.clone();
            Box::pin(update_seedtime_and_bonus_points(
                Arc::clone(&pool_2),
                seedtime_and_bonus_points_interval_secs,
                formula_sql,
            ))
        },
    )?;
    sched.add(seedtime_and_bonus_job).await?;

    // User torrent stats update task (seeding_size, seeding, leeching, snatched)
    let pool_4 = Arc::clone(&store.pool);
    let user_torrent_stats_job = Job::new_repeated_async(
        Duration::from_secs(store.env.periodic_tasks.user_torrent_stats_update_seconds),
        move |_uuid, _l| Box::pin(update_user_torrent_stats(Arc::clone(&pool_4))),
    )?;
    sched.add(user_torrent_stats_job).await?;

    // Inactive user ban task
    let pool_5 = Arc::clone(&store.pool);
    let inactive_user_ban_job = Job::new_repeated_async(
        Duration::from_secs(store.env.periodic_tasks.inactive_user_ban_seconds),
        move |_uuid, _l| Box::pin(ban_inactive_users(Arc::clone(&pool_5))),
    )?;
    sched.add(inactive_user_ban_job).await?;

    // Expired warnings cleanup task
    let pool_7 = Arc::clone(&store.pool);
    let expired_warnings_job = Job::new_repeated_async(
        Duration::from_secs(store.env.periodic_tasks.expired_warnings_seconds),
        move |_uuid, _l| Box::pin(clear_expired_warnings(Arc::clone(&pool_7))),
    )?;
    sched.add(expired_warnings_job).await?;

    // Artist peer stats aggregation task
    let pool_artist_peers = Arc::clone(&store.pool);
    let artist_peer_stats_job = Job::new_repeated_async(
        Duration::from_secs(store.env.periodic_tasks.artist_peer_stats_update_seconds),
        move |_uuid, _l| Box::pin(update_artist_peer_stats(Arc::clone(&pool_artist_peers))),
    )?;
    sched.add(artist_peer_stats_job).await?;

    // Materialized view refresh task
    let pool_6 = Arc::clone(&store.pool);
    let materialized_view_refresh_job = Job::new_repeated_async(
        Duration::from_secs(store.env.periodic_tasks.materialized_view_refresh_seconds),
        move |_uuid, _l| Box::pin(refresh_title_group_hierarchy_lite(Arc::clone(&pool_6))),
    )?;
    sched.add(materialized_view_refresh_job).await?;

    // let update_torrent_seeders_leechers_interval =
    //     env::var("TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS")
    //         .expect("env var TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS is missing");

    // let pool_1 = Arc::clone(&store.pool);
    // let job1 = match Job::new_async(
    //     update_torrent_seeders_leechers_interval.as_str(),
    //     move |_uuid, _l| Box::pin(update_torrent_seeders_leechers(Arc::clone(&pool_1))),
    // ) {
    //     Ok(job) => job,
    //     Err(e) => {
    //         return Err(format!(
    //             "Error creating job for updating torrents seeders and leechers: {e}"
    //         )
    //         .into());
    //     }
    // };
    // sched.add(job1).await?;

    // // this interval should be often enough
    // // let cleanup_interval_seconds = arc.tracker_announce_interval * 2;
    // let remove_inactive_peers_interval = env::var("TASK_INTERVAL_REMOVE_INACTIVE_PEERS")
    //     .expect("env var TASK_INTERVAL_REMOVE_INACTIVE_PEERS is missing");

    // // cleaning old peers is also done when the client sends a "stop" event
    // // but it doesn't always do it, so we need to clean the ones that are gone without sending this event
    // let pool_2 = Arc::clone(&store.pool);
    // let announce_interval = store.env.tracker.announce_interval;
    // let announce_interval_grace_period = store.env.tracker.announce_interval_grace_period;
    // let job2 = match Job::new_async(remove_inactive_peers_interval.as_str(), move |_uuid, _l| {
    //     Box::pin(remove_inactive_peers(
    //         Arc::clone(&pool_2),
    //         announce_interval,
    //         announce_interval_grace_period,
    //     ))
    // }) {
    //     Ok(job) => job,
    //     Err(e) => {
    //         return Err(format!("Error creating job for cleaning inactive peers: {e}").into());
    //     }
    // };
    // sched.add(job2).await?;

    sched.start().await?;

    Ok(sched)
}
