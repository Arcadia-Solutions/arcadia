use arcadia_common::error::announce::Error;
use sqlx::postgres::PgQueryResult;
use std::borrow::Borrow;

use crate::connection_pool::ConnectionPool;

#[derive(sqlx::FromRow)]
pub struct UserCompact {
    pub id: i64,
}

impl ConnectionPool {
    pub async fn find_user_with_passkey(
        &self,
        passkey_upper: i64,
        passkey_lower: i64,
    ) -> Result<UserCompact, Error> {
        sqlx::query_as!(
            UserCompact,
            r#"
                SELECT id FROM users
                WHERE (passkey_upper, passkey_lower) = ($1, $2)
            "#,
            passkey_upper,
            passkey_lower
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::InvalidPassKey)
    }

    pub async fn find_torrent_with_id(
        &self,
        info_hash: &[u8; 20],
    ) -> Result<TorrentCompact, Error> {
        sqlx::query_as!(
            TorrentCompact,
            r#"
                SELECT id, upload_factor, download_factor FROM torrents
                WHERE info_hash = $1
            "#,
            info_hash
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::InvalidInfoHash)
    }

    pub async fn credit_user_upload_download(
        &self,
        uploaded: i64,
        downloaded: i64,
        real_uploaded: i64,
        real_downloaded: i64,
        user_id: i64,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET uploaded = uploaded + $1,
                downloaded = downloaded + $2,
                real_uploaded = real_uploaded + $3,
                real_downloaded = real_downloaded + $4
            WHERE id = $5
            "#,
            uploaded,
            downloaded,
            real_uploaded,
            real_downloaded,
            user_id
        )
        .execute(self.borrow())
        .await
        .map_err(|_| Error::InvalidUserId)
    }

    pub async fn update_total_seedtime(
        &self,
        user_id: i64,
        torrent_id: i64,
        announce_interval: u32,
        grace_period: u32,
    ) -> Result<PgQueryResult, Error> {
        // normally, there should always already be an entry, added when the user snatched the torrent
        // but if they used the file from another user, and edited it with their own passkey, this will still work
        // this can be changed if it's not a desired behavior
        sqlx::query!(
            r#"
            INSERT INTO torrent_activities(torrent_id, user_id)
            VALUES ($1, $2)
            ON CONFLICT (torrent_id, user_id) DO UPDATE
            SET
                first_seen_seeding_at = CASE
                    WHEN torrent_activities.first_seen_seeding_at IS NULL
                    THEN NOW()
                    ELSE torrent_activities.first_seen_seeding_at
                END,
                total_seed_time = CASE
                    WHEN torrent_activities.last_seen_seeding_at > NOW() - ($3 || ' seconds')::INTERVAL
                    THEN torrent_activities.total_seed_time + EXTRACT(EPOCH FROM (NOW() - torrent_activities.last_seen_seeding_at))::BIGINT
                    ELSE torrent_activities.total_seed_time
                END,
                last_seen_seeding_at = NOW()
            "#,
            torrent_id,
            user_id,
            (announce_interval + grace_period).to_string()
        )
        .execute(self.borrow())
        .await
        .map_err(|_| Error::InvalidUserIdOrTorrentId)
    }
}

#[derive(sqlx::FromRow)]
pub struct TorrentCompact {
    pub id: i64,
    pub upload_factor: f64,
    pub download_factor: f64,
}
