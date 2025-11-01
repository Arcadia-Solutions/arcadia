use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Decode, PgPool};
use std::net::IpAddr;
use std::ops::{Deref, DerefMut};

use crate::tracker::models::peer::{self, Peer};
use crate::tracker::models::peer_id::PeerId;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub struct InfoHash(pub [u8; 20]);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Torrent {
    pub upload_factor: i16,
    pub download_factor: i16,
    pub seeders: u32,
    pub leechers: u32,
    pub times_completed: u32,
    pub is_deleted: bool,
    pub peers: peer::Map,
}

#[derive(Debug)]
pub struct Map(pub IndexMap<u32, Torrent>);

#[derive(Debug)]
pub struct DBImportTorrent {
    pub id: i32,
    pub upload_factor: i16,
    pub download_factor: i16,
    pub seeders: i64,
    pub leechers: i64,
    pub times_completed: i32,
    pub is_deleted: bool,
}

impl Map {
    pub async fn from_database(db: &PgPool) -> Self {
        let rows = sqlx::query_as!(
            DBImportTorrent,
            r#"
                    SELECT
                        id,
                        upload_factor,
                        download_factor,
                        seeders,
                        leechers,
                        times_completed,
                        CASE
                            WHEN deleted_at IS NOT NULL THEN TRUE
                            ELSE FALSE
                        END AS "is_deleted!"
                    FROM torrents
                    "#
        )
        .fetch_all(db)
        .await
        .expect("could not get torrents");

        let mut map: Map = Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            let torrent = Torrent {
                upload_factor: r.upload_factor,
                download_factor: r.download_factor,
                seeders: r.seeders as u32,
                leechers: r.leechers as u32,
                times_completed: r.times_completed as u32,
                is_deleted: r.is_deleted,
                peers: peer::Map::new(),
            };
            map.insert(r.id as u32, torrent);
        }

        // Load peers into each torrent
        let mut peers = sqlx::query!(
            r#"
                        SELECT
                            peers.ip as "ip_address: IpAddr",
                            peers.user_id as "user_id",
                            peers.torrent_id as "torrent_id",
                            peers.port as "port",
                            peers.seeder as "is_seeder: bool",
                            peers.active as "is_active: bool",
                            peers.updated_at as "updated_at: DateTime<Utc>",
                            peers.uploaded as "uploaded",
                            peers.downloaded as "downloaded",
                            peers.peer_id as "peer_id: PeerId"
                        FROM
                            peers
                    "#
        )
        .fetch(db);

        while let Some(peer) = peers.try_next().await.expect("Failed loading peers.") {
            map.entry(peer.torrent_id as u32).and_modify(|torrent| {
                torrent.peers.insert(
                    peer::Index {
                        user_id: peer.user_id as u32,
                        peer_id: peer.peer_id,
                    },
                    Peer {
                        ip_address: peer.ip_address,
                        port: peer.port as u16,
                        is_seeder: peer.is_seeder,
                        is_active: peer.is_active,
                        has_sent_completed: false,
                        updated_at: peer
                            .updated_at
                            .expect("Peer with a null updated_at found in database."),
                        uploaded: peer.uploaded as u64,
                        downloaded: peer.downloaded as u64,
                    },
                );
            });
        }

        map
    }
}

impl Deref for Map {
    type Target = IndexMap<u32, Torrent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'r, DB: Database> Decode<'r, DB> for InfoHash
where
    &'r [u8]: Decode<'r, DB>,
{
    /// Decodes the database's string representation of the 40 character long
    /// infohash in hex into a byte slice
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<InfoHash, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&[u8] as Decode<DB>>::decode(value)?;

        if value.len() != 20 {
            let error: Box<dyn std::error::Error + Send + Sync> =
                Box::new(crate::error::DecodeError::InfoHash);

            return Err(error);
        }

        Ok(InfoHash(<[u8; 20]>::try_from(&value[0..20])?))
    }
}
