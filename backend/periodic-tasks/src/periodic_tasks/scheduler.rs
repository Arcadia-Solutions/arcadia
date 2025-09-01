use std::{env, sync::Arc};
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::{periodic_tasks::peers::remove_inactive_peers, store::Store};

use super::torrents::update_torrent_seeders_leechers;

pub async fn run_periodic_tasks(store: Arc<Store>) -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    let update_torrent_seeders_leechers_interval =
        env::var("TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS")
            .expect("env var TASK_INTERVAL_UPDATE_TORRENT_SEEDERS_LEECHERS is missing");

    let pool_1 = Arc::clone(&store.pool);
    let job1 = match Job::new_async(
        update_torrent_seeders_leechers_interval.as_str(),
        move |_uuid, _l| Box::pin(update_torrent_seeders_leechers(Arc::clone(&pool_1))),
    ) {
        Ok(job) => job,
        Err(e) => {
            return Err(format!(
                "Error creating job for updating torrents seeders and leechers: {e}"
            )
            .into());
        }
    };
    sched.add(job1).await?;

    // this interval should be often enough
    // let cleanup_interval_seconds = arc.tracker_announce_interval * 2;
    let remove_inactive_peers_interval = env::var("TASK_INTERVAL_REMOVE_INACTIVE_PEERS")
        .expect("env var TASK_INTERVAL_REMOVE_INACTIVE_PEERS is missing");

    // cleaning old peers is also done when the client sends a "stop" event
    // but it doesn't always do it, so we need to clean the ones that are gone without sending this event
    let pool_2 = Arc::clone(&store.pool);
    let announce_interval = store.env.tracker.announce_interval;
    let announce_interval_grace_period = store.env.tracker.announce_interval_grace_period;
    let job2 = match Job::new_async(remove_inactive_peers_interval.as_str(), move |_uuid, _l| {
        Box::pin(remove_inactive_peers(
            Arc::clone(&pool_2),
            announce_interval,
            announce_interval_grace_period,
        ))
    }) {
        Ok(job) => job,
        Err(e) => {
            return Err(format!("Error creating job for cleaning inactive peers: {e}").into());
        }
    };
    sched.add(job2).await?;

    sched.start().await?;

    Ok(())
}
