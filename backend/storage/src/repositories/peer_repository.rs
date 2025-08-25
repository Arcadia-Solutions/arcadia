use crate::connection_pool::ConnectionPool;
use crate::models::peer::PeerStatus;
use arcadia_common::error::Result;
use arcadia_common::models::tracker::announce::{Announce, Peer};
use sqlx::types::ipnetwork::IpNetwork;
use std::borrow::Borrow;

use crate::models;

impl ConnectionPool {
    pub async fn get_user_peers(&self, user_id: i64) -> Vec<models::peer::Peer> {
        sqlx::query_as!(
            models::peer::Peer,
            r#"
                    SELECT
                        ip,
                        port,
                        user_agent,
                        MIN(first_seen_at) as "first_seen_at!",
                        MAX(last_seen_at) as "last_seen_at!",
                        SUM(real_uploaded)::BIGINT as "real_uploaded!",
                        SUM(real_downloaded)::BIGINT as "real_downloaded!",
                        status::peer_status_enum as "status!: PeerStatus"
                    FROM peers
                    WHERE user_id = $1
                    GROUP BY (peer_id, ip, port, user_agent, status)
                "#,
            user_id
        )
        .fetch_all(self.borrow())
        .await
        .expect("failed to retrieve peers")
    }

    pub async fn remove_peer(
        &self,
        torrent_id: &i64,
        peer_id: &[u8; 20],
        ip: &IpNetwork,
        port: u16,
    ) {
        sqlx::query!(
            r#"
                DELETE FROM peers WHERE
                (torrent_id, peer_id, ip, port) = ($1, $2, $3, $4)
            "#,
            torrent_id,
            peer_id,
            ip,
            port as i32
        )
        .execute(self.borrow())
        .await
        .expect("failed removing peer from table");
    }

    // returns uploaded/downloaded before the update
    pub async fn insert_or_update_peer(
        &self,
        torrent_id: &i64,
        ip: &IpNetwork,
        user_id: &i64,
        ann: &Announce,
        user_agent: Option<&str>,
    ) -> (i64, i64) {
        let existing = sqlx::query!(
            r#"
            SELECT real_uploaded, real_downloaded
            FROM peers
            WHERE torrent_id = $1 AND peer_id = $2 AND ip = $3 AND port = $4 AND user_id = $5
            "#,
            torrent_id,
            &ann.peer_id,
            ip,
            ann.port as i32,
            user_id
        )
        .fetch_optional(self.borrow())
        .await
        .expect("failed");

        let peer_status = if ann.left.unwrap() == 0 {
            PeerStatus::Seeding
        } else {
            PeerStatus::Leeching
        };

        sqlx::query!(
            r#"
            INSERT INTO peers(torrent_id, peer_id, ip, port, user_id, real_uploaded, real_downloaded, user_agent, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::peer_status_enum)
            ON CONFLICT (torrent_id, peer_id, ip, port) DO UPDATE
            SET
                last_seen_at = NOW(),
                real_uploaded = $6,
                real_downloaded = $7,
                status = $9::peer_status_enum
            "#,
            torrent_id,
            &ann.peer_id,
            ip,
            ann.port as i32,
            user_id,
            ann.uploaded.unwrap_or(0) as i64,
            ann.downloaded.unwrap_or(0) as i64,
            user_agent,
            peer_status as PeerStatus
        )
        .execute(self.borrow())
        .await
        .expect("failed");

        existing
            .map(|row| (row.real_uploaded, row.real_downloaded))
            .unwrap_or((0, 0))
    }

    pub async fn find_torrent_peers(&self, torrent_id: &i64, user_id: &i64) -> Vec<Peer> {
        let peers = sqlx::query!(
            r#"
            SELECT peers.ip AS ip, peers.port AS port
            FROM peers
            WHERE
                torrent_id = $1
            AND
                peers.user_id != $2
            "#,
            torrent_id,
            user_id
        )
        .fetch_all(self.borrow())
        .await
        .expect("failed");

        peers
            .into_iter()
            .map(|p| {
                let std::net::IpAddr::V4(ipv4) = p.ip.ip() else {
                    panic!("oops");
                };

                Peer {
                    ip: ipv4,
                    port: p.port as u16,
                }
            })
            .collect::<Vec<_>>()
    }

    pub async fn remove_inactive_peers(&self, seconds_since_last_announce: f64) -> Result<u64> {
        let removed_peers_amount = sqlx::query!(
            r#"DELETE FROM peers WHERE last_seen_at < NOW() - INTERVAL '1 second' * $1"#,
            seconds_since_last_announce
        )
        .execute(self.borrow())
        .await?
        .rows_affected();

        Ok(removed_peers_amount)
    }
}
