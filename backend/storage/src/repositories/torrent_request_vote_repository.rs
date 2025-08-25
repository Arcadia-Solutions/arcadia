use crate::{
    connection_pool::ConnectionPool,
    models::{
        torrent_request_vote::{TorrentRequestVote, UserCreatedTorrentRequestVote},
        user::User,
    },
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_torrent_request_vote(
        &self,
        torrent_request_vote: &UserCreatedTorrentRequestVote,
        current_user: &User,
    ) -> Result<TorrentRequestVote> {
        if current_user.bonus_points - torrent_request_vote.bounty_bonus_points < 0 {
            return Err(Error::InsufficientBonusPointsForBounty);
        }
        if current_user.uploaded - torrent_request_vote.bounty_upload < 0 {
            return Err(Error::InsufficientUploadForBounty);
        }
        // TODO config: check if the bounty is above the minimum set in the config
        // TODO config: check if the user's ratio stays above the minimum ratio set in the config (after the uploaded amount changes)

        let created_torrent_request_vote = sqlx::query_as!(
            TorrentRequestVote,
            r#"
                WITH inserted_vote AS (
                    INSERT INTO torrent_request_votes (torrent_request_id, created_by_id,
                                                      bounty_upload, bounty_bonus_points)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                ),
                updated_user AS (
                    UPDATE users u
                    SET
                        uploaded = u.uploaded - $3,
                        bonus_points = u.bonus_points - $4
                    WHERE u.id = (SELECT created_by_id FROM inserted_vote)
                )
                SELECT
                    inserted_vote.*
                FROM inserted_vote
            "#,
            torrent_request_vote.torrent_request_id,
            current_user.id,
            torrent_request_vote.bounty_upload,
            torrent_request_vote.bounty_bonus_points
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateTorrentRequestVote)?;

        Ok(created_torrent_request_vote)
    }
}
