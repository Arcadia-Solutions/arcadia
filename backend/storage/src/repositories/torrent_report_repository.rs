use crate::{
    connection_pool::ConnectionPool,
    models::torrent_report::{TorrentReport, UserCreatedTorrentReport},
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn report_torrent(
        &self,
        form: &UserCreatedTorrentReport,
        user_id: i32,
    ) -> Result<TorrentReport> {
        let torrent_report = sqlx::query_as!(
            TorrentReport,
            r#"
                INSERT INTO torrent_reports (reported_by_id, reported_torrent_id, description)
                VALUES ($1, $2, $3)
                RETURNING id, reported_at, reported_by_id, reported_torrent_id, description
            "#,
            user_id,
            form.reported_torrent_id,
            form.description,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateTorrentReport)?;

        Ok(torrent_report)
    }

    pub async fn delete_torrent_report(&self, torrent_report_id: i64) -> Result<()> {
        sqlx::query!(
            r#"
                DELETE FROM torrent_reports
                WHERE id = $1
            "#,
            torrent_report_id,
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotDeleteTorrentReport)?;

        Ok(())
    }

    pub async fn get_torrent_report_by_id(
        &self,
        torrent_report_id: i64,
    ) -> Result<Option<TorrentReport>> {
        let torrent_report = sqlx::query_as!(
            TorrentReport,
            r#"
                SELECT id, reported_at, reported_by_id, reported_torrent_id, description
                FROM torrent_reports
                WHERE id = $1
            "#,
            torrent_report_id,
        )
        .fetch_optional(self.borrow())
        .await
        .map_err(Error::CouldNotGetTorrentReport)?;

        Ok(torrent_report)
    }
}
