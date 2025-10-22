use std::sync::Arc;

use arcadia_shared::tracker::models::Flushable;
use tokio::join;

use crate::Tracker;

pub async fn handle(arc: &Arc<Tracker>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(1));
    let mut counter = 0_u64;

    loop {
        interval.tick().await;
        counter += 1;

        if counter % arc.env.flush_interval_milliseconds == 0 {
            flush(arc).await;
        }

        // if counter % (arc.env.peer_expiry_interval * 1000) == 0 {
        //     reap(tracker).await;
        // }
    }
}

pub async fn flush(arc: &Arc<Tracker>) {
    let user_updates_guard = arc.user_updates.lock().await;
    join!(user_updates_guard.flush_to_backend());
}
