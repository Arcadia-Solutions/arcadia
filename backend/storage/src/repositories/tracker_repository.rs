use crate::connection_pool::ConnectionPool;
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::user::{Passkey, User};
use std::borrow::Borrow;

// This file contains functions for Arcadia's tracker
// but not necessarily related to the tracker itself directly

impl ConnectionPool {
    pub async fn find_users(&self) -> Result<Vec<User>> {
        // TODO: fix this
        // query_as!() doesn't work as it requires the FromString trait
        // which is implemented, but somehow still throws an error
        let rows = sqlx::query!(
            r#"
                SELECT
                    id,
                    passkey,
                    TRUE AS "can_download!",
                    0::int4 AS "num_seeding!",
                    0::int4 AS "num_leeching!"
                FROM users
            "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get users");

        let users = rows
            .into_iter()
            .map(|r| User {
                id: r.id as u32,
                passkey: r
                    .passkey
                    .parse::<Passkey>()
                    .expect("invalid passkey in database"),
                can_download: r.can_download,
                num_seeding: r.num_seeding as u32,
                num_leeching: r.num_leeching as u32,
            })
            .collect();

        Ok(users)
    }
}
