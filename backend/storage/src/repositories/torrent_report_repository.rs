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
        user_id: i64,
    ) -> Result<TorrentReport> {
        let torrent_report = sqlx::query_as!(
            TorrentReport,
            r#"
                INSERT INTO torrent_reports (reported_by_id, reported_torrent_id, description)
                VALUES ($1, $2, $3)
                RETURNING *
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
}
