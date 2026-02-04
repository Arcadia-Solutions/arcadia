use std::collections::HashSet;

use arcadia_shared::tracker::models::{
    env::SnatchedTorrentBonusPointsTransferredTo, peer_id::PeerId,
};
use sqlx::PgPool;

use crate::announce::error::AnnounceError;

/// Check and deduct bonus points snatch cost for a new leech.
///
/// This function performs atomic queries that:
/// 1. Gets the torrent's bonus_points_snatch_cost and uploader ID
/// 2. Checks if user already has a torrent_activities row where they were leeching (downloaded > 0)
/// 3. Deducts points if: cost > 0, user is not uploader, no existing leeching activity, has enough points
/// 4. Optionally transfers the deducted points to uploader or current seeders
pub async fn check_and_deduct_snatch_cost(
    pool: &PgPool,
    torrent_id: u32,
    user_id: u32,
    transfer_to: Option<&SnatchedTorrentBonusPointsTransferredTo>,
) -> Result<(), AnnounceError> {
    let mut tx = pool.begin().await.map_err(|e| {
        log::error!("Failed to begin transaction: {}", e);
        AnnounceError::InternalTrackerError
    })?;

    let row = sqlx::query!(
        r#"
        WITH torrent_info AS (
            SELECT bonus_points_snatch_cost, created_by_id
            FROM torrents WHERE id = $1
        ),
        existing_leeching_activity AS (
            SELECT 1 FROM torrent_activities
            WHERE torrent_id = $1 AND user_id = $2 AND downloaded > 0
        ),
        deduction AS (
            UPDATE users SET bonus_points = bonus_points - (SELECT bonus_points_snatch_cost FROM torrent_info)
            WHERE id = $2
              AND (SELECT bonus_points_snatch_cost FROM torrent_info) > 0
              AND $2 != (SELECT created_by_id FROM torrent_info)
              AND bonus_points >= (SELECT bonus_points_snatch_cost FROM torrent_info)
              -- we do this check in case the user only partially downloaded the torrent, sent a stopped event, and started leeching again
              -- the peer is removed from the in-memory db at a stopped event, and would be considered a new leecher
              AND NOT EXISTS (SELECT 1 FROM existing_leeching_activity)
            RETURNING id
        )
        SELECT
            (SELECT bonus_points_snatch_cost FROM torrent_info) AS cost,
            (SELECT created_by_id FROM torrent_info) AS uploader_id,
            EXISTS (SELECT 1 FROM deduction) AS deducted,
            EXISTS (SELECT 1 FROM existing_leeching_activity) AS has_existing_leeching_activity
        "#,
        torrent_id as i32,
        user_id as i32,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Failed to check/deduct bonus points: {}", e);
        AnnounceError::InternalTrackerError
    })?;

    let cost = row.cost.unwrap_or(0);
    let is_uploader = row
        .uploader_id
        .map(|id| id as u32 == user_id)
        .unwrap_or(false);
    let deducted = row.deducted.unwrap_or(false);
    let has_existing_leeching_activity = row.has_existing_leeching_activity.unwrap_or(false);

    // If cost > 0, user is not uploader, no existing leeching activity, and deduction failed = not enough BP
    if cost > 0 && !is_uploader && !has_existing_leeching_activity && !deducted {
        return Err(AnnounceError::InsufficientBonusPoints(cost));
    }

    // Transfer bonus points if deduction happened and transfer is configured
    if deducted {
        match transfer_to {
            Some(SnatchedTorrentBonusPointsTransferredTo::Uploader) => {
                sqlx::query!(
                    r#"
                    UPDATE users SET bonus_points = bonus_points + $1
                    WHERE id = (SELECT created_by_id FROM torrents WHERE id = $2)
                    "#,
                    cost,
                    torrent_id as i32,
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    log::error!("Failed to transfer bonus points to uploader: {}", e);
                    AnnounceError::InternalTrackerError
                })?;
            }
            Some(SnatchedTorrentBonusPointsTransferredTo::CurrentSeeders) => {
                sqlx::query!(
                    r#"
                    WITH seeder_info AS (
                        SELECT DISTINCT user_id, COUNT(*) OVER () as seeder_count
                        FROM peers
                        WHERE torrent_id = $1 AND seeder = true AND active = true
                    )
                    UPDATE users SET bonus_points = bonus_points + $2 / (SELECT seeder_count FROM seeder_info LIMIT 1)
                    WHERE id IN (SELECT user_id FROM seeder_info)
                    "#,
                    torrent_id as i32,
                    cost,
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| {
                    log::error!("Failed to transfer bonus points to seeders: {}", e);
                    AnnounceError::InternalTrackerError
                })?;
            }
            None => {}
        }
    }

    tx.commit().await.map_err(|e| {
        log::error!("Failed to commit transaction: {}", e);
        AnnounceError::InternalTrackerError
    })?;

    Ok(())
}

pub fn is_torrent_client_allowed(
    peer_id: &PeerId,
    allowed_torrent_clients: &HashSet<Vec<u8>>,
) -> bool {
    let peer_id_without_hyphen = &peer_id.0[1..];
    for prefix in allowed_torrent_clients.iter() {
        if peer_id_without_hyphen.starts_with(prefix) {
            return true;
        }
    }
    false
}
