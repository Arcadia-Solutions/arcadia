use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sqlx::{types::ipnetwork::IpNetwork, PgPool};

use crate::{
    error::Error,
    tracker::models::{peer_id::PeerId, Flushable, Mergeable, Queue},
};

// Fields must be in same order as database primary key
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Index {
    pub user_id: u32,
    pub torrent_id: u32,
    pub peer_id: PeerId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerUpdate {
    pub ip: std::net::IpAddr,
    pub port: u16,
    pub agent: String,
    pub uploaded: u64,
    pub downloaded: u64,
    pub is_active: bool,
    pub is_seeder: bool,
    pub left: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Mergeable for PeerUpdate {
    fn merge(&mut self, new: &Self) {
        if new.updated_at > self.updated_at {
            self.ip = new.ip;
            self.port = new.port;
            self.agent = new.agent.clone();
            self.uploaded = new.uploaded;
            self.downloaded = new.downloaded;
            self.is_active = new.is_active;
            self.is_seeder = new.is_seeder;
            self.left = new.left;
            self.updated_at = new.updated_at;
        }

        self.created_at = std::cmp::min(self.created_at, new.created_at);
    }
}

impl Flushable<PeerUpdate> for Mutex<Queue<Index, PeerUpdate>> {
    async fn flush_to_database(&self, db: &PgPool) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, PeerUpdate)>>();
        if updates.is_empty() {
            return;
        }

        let mut user_ids: Vec<i32> = Vec::with_capacity(updates.len());
        let mut torrent_ids: Vec<i32> = Vec::with_capacity(updates.len());
        let mut peer_ids: Vec<Vec<u8>> = Vec::with_capacity(updates.len());
        let mut ips: Vec<IpNetwork> = Vec::with_capacity(updates.len());
        let mut ports: Vec<i32> = Vec::with_capacity(updates.len());
        let mut agents: Vec<String> = Vec::with_capacity(updates.len());
        let mut uploadeds: Vec<i64> = Vec::with_capacity(updates.len());
        let mut downloadeds: Vec<i64> = Vec::with_capacity(updates.len());
        let mut lefts: Vec<i64> = Vec::with_capacity(updates.len());
        let mut actives: Vec<bool> = Vec::with_capacity(updates.len());
        let mut seeders: Vec<bool> = Vec::with_capacity(updates.len());
        let mut created_ats: Vec<DateTime<Utc>> = Vec::with_capacity(updates.len());
        let mut updated_ats: Vec<DateTime<Utc>> = Vec::with_capacity(updates.len());

        for (index, update) in updates {
            user_ids.push(index.user_id as i32);
            torrent_ids.push(index.torrent_id as i32);
            peer_ids.push(index.peer_id.to_vec());
            ips.push(IpNetwork::from(update.ip));
            ports.push(update.port as i32);
            agents.push(update.agent.clone());
            uploadeds.push(update.uploaded as i64);
            downloadeds.push(update.downloaded as i64);
            lefts.push(update.left as i64);
            actives.push(update.is_active);
            seeders.push(update.is_seeder);
            created_ats.push(update.created_at);
            updated_ats.push(update.updated_at);
        }

        let result = sqlx::query!(
            r#"
                INSERT INTO peers (
                    peer_id,
                    ip,
                    port,
                    agent,
                    uploaded,
                    downloaded,
                    "left",
                    active,
                    seeder,
                    created_at,
                    updated_at,
                    torrent_id,
                    user_id
                )
                SELECT
                    t.peer_id,
                    t.ip,
                    t.port,
                    t.agent,
                    t.uploaded,
                    t.downloaded,
                    t."left",
                    t.active,
                    t.seeder,
                    -- stored as timestamp without time zone in DB
                    (t.created_at AT TIME ZONE 'UTC')::timestamp,
                    (t.updated_at AT TIME ZONE 'UTC')::timestamp,
                    t.torrent_id,
                    t.user_id
                FROM (
                    SELECT * FROM unnest(
                        $1::bytea[],
                        $2::inet[],
                        $3::int[],
                        $4::varchar[],
                        $5::bigint[],
                        $6::bigint[],
                        $7::bigint[],
                        $8::boolean[],
                        $9::boolean[],
                        $10::timestamptz[],
                        $11::timestamptz[],
                        $12::int[],
                        $13::int[]
                    ) AS t(
                        peer_id,
                        ip,
                        port,
                        agent,
                        uploaded,
                        downloaded,
                        "left",
                        active,
                        seeder,
                        created_at,
                        updated_at,
                        torrent_id,
                        user_id
                    )
                ) AS t
                ON CONFLICT (user_id, torrent_id, peer_id) DO UPDATE SET
                    ip = EXCLUDED.ip,
                    port = EXCLUDED.port,
                    agent = EXCLUDED.agent,
                    uploaded = EXCLUDED.uploaded,
                    downloaded = EXCLUDED.downloaded,
                    "left" = EXCLUDED."left",
                    active = EXCLUDED.active,
                    seeder = EXCLUDED.seeder,
                    updated_at = EXCLUDED.updated_at
            "#,
            &peer_ids,
            &ips,
            &ports,
            &agents,
            &uploadeds,
            &downloadeds,
            &lefts,
            &actives,
            &seeders,
            &created_ats,
            &updated_ats,
            &torrent_ids,
            &user_ids
        )
        .execute(db)
        .await
        .map_err(|e| Error::DatabseError(e.to_string()));

        if result.is_err() {
            // TODO: reinsert the updates that failed and retry
            panic!("Failed to insert peer updates: {}", result.err().unwrap());
        } else {
            log::info!("Inserted {amount_of_updates} peer updates");
        }
    }
}
