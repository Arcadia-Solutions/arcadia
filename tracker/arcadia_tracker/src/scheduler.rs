use actix_web::web::Data;
use arcadia_shared::tracker::models::{
    peer::remove_peers_from_database,
    peer_update,
    torrent_update::{self, TorrentUpdate},
    Flushable,
};
use chrono::{Duration, Utc};
use tokio::join;

use crate::Tracker;

pub async fn handle(arc: &Data<Tracker>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(1));
    let mut counter = 0_u64;

    loop {
        interval.tick().await;
        counter += 1;

        if counter % arc.env.flush_interval_milliseconds == 0 {
            flush(arc).await;
        }

        if counter % (arc.env.peer_expiry_interval * 1000) == 0 {
            reap(arc).await;
        }
    }
}

pub async fn flush(arc: &Data<Tracker>) {
    join!(
        arc.user_updates.flush_to_database(&arc.pool),
        arc.torrent_updates.flush_to_database(&arc.pool),
        arc.peer_updates.flush_to_database(&arc.pool)
    );
}

/// Remove peers that have not announced for some time
pub async fn reap(arc: &Data<Tracker>) {
    let ttl = Duration::seconds(arc.env.active_peer_ttl.try_into().unwrap());
    let active_cutoff = Utc::now().checked_sub_signed(ttl).unwrap();
    let ttl = Duration::seconds(arc.env.inactive_peer_ttl.try_into().unwrap());
    let inactive_cutoff = Utc::now().checked_sub_signed(ttl).unwrap();
    let mut all_removed_peers: Vec<peer_update::Index> = Vec::new();

    for (torrent_id, torrent) in arc.torrents.lock().iter_mut() {
        let mut seeder_delta: i32 = 0;
        let mut leecher_delta: i32 = 0;

        // If a peer is marked as inactive and it has not announced for
        // more than inactive_peer_ttl, then it is permanently deleted.
        let torrent_removed_peers = torrent
            .peers
            .extract_if(.., |_index, peer| {
                inactive_cutoff <= peer.updated_at || peer.is_active
            })
            .map(|(index, _peer)| index)
            .collect::<Vec<arcadia_shared::tracker::models::peer::Index>>();
        all_removed_peers.extend(
            torrent_removed_peers
                .into_iter()
                .map(|index| peer_update::Index {
                    torrent_id: *torrent_id,
                    peer_id: index.peer_id,
                    user_id: index.user_id,
                })
                .collect::<Vec<peer_update::Index>>(),
        );

        for (index, peer) in torrent.peers.iter_mut() {
            // Peers get marked as inactive if not announced for more than
            // active_peer_ttl seconds. User peer count and torrent peer
            // count are updated to reflect.
            if peer.updated_at < active_cutoff && peer.is_active {
                peer.is_active = false;

                arc.users.write().entry(index.user_id).and_modify(|user| {
                    if peer.is_seeder {
                        user.num_seeding = user.num_seeding.saturating_sub(1);
                    } else {
                        user.num_leeching = user.num_leeching.saturating_sub(1);
                    }
                });
                match peer.is_seeder {
                    true => seeder_delta -= 1,
                    false => leecher_delta -= 1,
                }
            }
        }

        // Update peer count of torrents and users
        if seeder_delta != 0 || leecher_delta != 0 {
            torrent.seeders = torrent.seeders.saturating_add_signed(seeder_delta);
            torrent.leechers = torrent.leechers.saturating_add_signed(leecher_delta);

            arc.torrent_updates.lock().upsert(
                torrent_update::Index {
                    torrent_id: torrent_id.to_owned(),
                },
                TorrentUpdate {
                    seeder_delta,
                    leecher_delta,
                    times_completed_delta: 0,
                },
            );
        }
    }

    remove_peers_from_database(&arc.pool, &all_removed_peers).await;
}
