use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::Error,
    tracker::models::{Flushable, Mergeable, Queue},
};

// Fields must be in same order as database primary key
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Index {
    pub user_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdate {
    pub uploaded_delta: u64,
    pub downloaded_delta: u64,
    pub real_uploaded_delta: u64,
    pub real_downloaded_delta: u64,
}

impl Mergeable for UserUpdate {
    fn merge(&mut self, new: &Self) {
        self.uploaded_delta = self.uploaded_delta.saturating_add(new.uploaded_delta);
        self.downloaded_delta = self.downloaded_delta.saturating_add(new.downloaded_delta);
        self.real_uploaded_delta = self
            .real_uploaded_delta
            .saturating_add(new.real_uploaded_delta);
        self.real_downloaded_delta = self
            .real_downloaded_delta
            .saturating_add(new.real_downloaded_delta);
    }
}

impl Flushable<UserUpdate> for Mutex<Queue<Index, UserUpdate>> {
    async fn flush_to_database(&self, db: &PgPool) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, UserUpdate)>>();
        if updates.is_empty() {
            return;
        }

        let mut user_ids = Vec::new();
        let mut uploaded_deltas = Vec::new();
        let mut downloaded_deltas = Vec::new();
        let mut real_uploaded_deltas = Vec::new();
        let mut real_downloaded_deltas = Vec::new();

        for (index, update) in updates {
            user_ids.push(index.user_id as i32);
            uploaded_deltas.push(update.uploaded_delta as i64);
            downloaded_deltas.push(update.downloaded_delta as i64);
            real_uploaded_deltas.push(update.real_uploaded_delta as i64);
            real_downloaded_deltas.push(update.real_downloaded_delta as i64);
        }

        let result = sqlx::query!(
                    r#"
                        UPDATE users
                        SET
                            uploaded = uploaded + updates.uploaded_delta,
                            downloaded = downloaded + updates.downloaded_delta,
                            real_uploaded = real_uploaded + updates.real_uploaded_delta,
                            real_downloaded = real_downloaded + updates.real_downloaded_delta
                        FROM (
                            SELECT * FROM unnest($1::int[], $2::bigint[], $3::bigint[], $4::bigint[], $5::bigint[]) AS
                            t(user_id, uploaded_delta, downloaded_delta, real_uploaded_delta, real_downloaded_delta)
                        ) AS updates
                        WHERE users.id = updates.user_id
                    "#,
                    &user_ids,
                    &uploaded_deltas,
                    &downloaded_deltas,
                    &real_uploaded_deltas,
                    &real_downloaded_deltas
                )
                .execute(db)
                .await.map_err(|e| Error::DatabseError(e.to_string()));

        if result.is_err() {
            // TODO: reinsert the updates that failed and retry
            panic!("failed inserting user updates: {}", result.err().unwrap());
        } else {
            log::info!("Inserted {amount_of_updates} user updates");
        }
    }
}
