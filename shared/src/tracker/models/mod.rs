use ringmap::RingMap;
use serde::{de::DeserializeOwned, Serialize};
use sqlx::PgPool;
use std::hash::Hash;

pub mod env;
pub mod infohash_2_id;
pub mod passkey_2_id;
pub mod peer;
pub mod peer_id;
pub mod peer_update;
pub mod torrent;
pub mod torrent_activity_update;
pub mod torrent_update;
pub mod user;
pub mod user_update;

#[derive(Debug, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct Queue<K, V>
where
    K: Serialize + DeserializeOwned + Eq + Hash,
    V: Serialize + DeserializeOwned,
{
    #[bincode(with_serde)]
    pub records: RingMap<K, V>,
}

pub trait Mergeable {
    /// Merge an existing record with a new record
    fn merge(&mut self, new: &Self);
}

#[allow(async_fn_in_trait)]
pub trait Flushable<T> {
    /// Flushes updates to postgresql database
    async fn flush_to_database(&self, db: &PgPool);
}

impl<K, V> Queue<K, V>
where
    K: Hash + Eq + Ord + Serialize + DeserializeOwned,
    V: Clone + Mergeable + Serialize + DeserializeOwned,
{
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

impl<K, V> Default for Queue<K, V>
where
    K: Hash + Eq + Ord + Serialize + DeserializeOwned,
    V: Clone + Mergeable + Serialize + DeserializeOwned,
{
    fn default() -> Self {
        Self {
            records: RingMap::new(),
        }
    }
}

impl<K, V> Queue<K, V>
where
    K: Hash + Eq + Ord + Serialize + DeserializeOwned,
    V: Clone + Mergeable + Serialize + DeserializeOwned,
{
    /// Upsert a single update into the queue
    pub fn upsert(&mut self, key: K, value: V) {
        self.records
            .entry(key)
            .and_modify(|update| update.merge(&value))
            .or_push_back(value);
    }
}
