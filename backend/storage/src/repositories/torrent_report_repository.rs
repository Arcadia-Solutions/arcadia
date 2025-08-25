use crate::{
    connection_pool::ConnectionPool,
    models::{
        torrent_report::{TorrentReport, UserCreatedTorrentReport},
        user::User,
    },
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn report_torrent(
        &self,
        form: &UserCreatedTorrentReport,
        current_user: &User,
    ) -> Result<TorrentReport> {
        let torrent_report = sqlx::query_as!(
            TorrentReport,
            r#"
                INSERT INTO torrent_reports (reported_by_id, reported_torrent_id, description)
                VALUES ($1, $2, $3)
                RETURNING *
            "#,
            current_user.id,
            form.reported_torrent_id,
            form.description,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateTorrentReport)?;

        Ok(torrent_report)
    }
}
