use crate::{
    error::Error,
    tracker::models::{Flushable, Mergeable, Queue},
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// Fields must be in same order as database primary key
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Index {
    pub torrent_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorrentUpdate {
    pub seeder_delta: i32,
    pub leecher_delta: i32,
    pub times_completed_delta: u32,
}

impl Mergeable for TorrentUpdate {
    fn merge(&mut self, new: &Self) {
        self.seeder_delta = self.seeder_delta.saturating_add(new.seeder_delta);
        self.leecher_delta = self.leecher_delta.saturating_add(new.leecher_delta);
        self.times_completed_delta = self
            .times_completed_delta
            .saturating_add(new.times_completed_delta);
    }
}

impl Flushable<TorrentUpdate> for Mutex<Queue<Index, TorrentUpdate>> {
    async fn flush_to_database(&self, db: &PgPool) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, TorrentUpdate)>>();
        if updates.is_empty() {
            return;
        }
        let mut torrent_ids = Vec::new();
        let mut seeder_deltas = Vec::new();
        let mut leecher_deltas = Vec::new();
        let mut times_completed_deltas = Vec::new();

        for (index, update) in updates {
            torrent_ids.push(index.torrent_id as i32);
            seeder_deltas.push(update.seeder_delta as i64);
            leecher_deltas.push(update.leecher_delta as i64);
            times_completed_deltas.push(update.times_completed_delta as i64);
        }

        let result = sqlx::query!(
                    r#"
                        UPDATE torrents
                        SET
                            seeders = seeders + updates.seeder_delta,
                            leechers = leechers + updates.leecher_delta,
                            times_completed = times_completed + updates.times_completed_delta
                        FROM (
                            SELECT * FROM unnest($1::int[], $2::bigint[], $3::bigint[], $4::bigint[]) AS
                            t(torrent_id, seeder_delta, leecher_delta, times_completed_delta)
                        ) AS updates
                        WHERE torrents.id = updates.torrent_id
                    "#,
                    &torrent_ids,
                    &seeder_deltas,
                    &leecher_deltas,
                    &times_completed_deltas,
                )
                .execute(db)
                .await
                .map_err(|e| Error::DatabseError(e.to_string()));

        if result.is_err() {
            // TODO: reinsert the updates that failed and retry
            panic!(
                "Failed to insert torrent updates: {}",
                result.err().unwrap()
            );
        } else {
            log::info!("Inserted {amount_of_updates} torrent updates");
        }
    }
}
