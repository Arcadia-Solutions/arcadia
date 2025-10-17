use crate::connection_pool::ConnectionPool;
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::{torrent::Torrent, user::Passkey, user::User};
use std::borrow::Borrow;
use std::collections::HashMap;

// This file contains functions for Arcadia's tracker
// but not necessarily related to the tracker itself directly

impl ConnectionPool {
    pub async fn find_users(&self) -> Result<HashMap<u32, User>> {
        // TODO: fix this
        // query_as!() doesn't work as it requires the FromString trait
        // which is implemented, but somehow still throws an error
        let rows = sqlx::query!(
            r#"
                SELECT
                    id,
                    passkey as "passkey: Passkey",
                    TRUE AS "can_download!",
                    0::int4 AS "num_seeding!",
                    0::int4 AS "num_leeching!"
                FROM users
            "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get users");

        let mut map: HashMap<u32, User> = HashMap::with_capacity(rows.len());
        for r in rows {
            let id = r.id as u32;
            let user = User {
                can_download: r.can_download,
                num_seeding: r.num_seeding as u32,
                num_leeching: r.num_leeching as u32,
            };
            map.insert(id, user);
        }

        Ok(map)
    }

    pub async fn find_torrents(&self) -> Result<HashMap<u32, Torrent>> {
        // TODO: fix this
        // query_as!() doesn't work as it requires the FromString trait
        // which is implemented, but somehow still throws an error
        let rows = sqlx::query!(
            r#"
                SELECT
                    id,
                    upload_factor,
                    download_factor,
                    seeders,
                    leechers,
                    completed
                FROM torrents
            "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get torrents");

        let mut map: HashMap<u32, Torrent> = HashMap::with_capacity(rows.len());
        for r in rows {
            let id = r.id as u32;
            let torrent = Torrent {
                upload_factor: r.upload_factor,
                download_factor: r.download_factor,
                seeders: r.seeders,
                leechers: r.leechers,
                completed: r.completed,
            };
            map.insert(id, torrent);
        }

        Ok(map)
    }
}
