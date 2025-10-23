use std::{
    future::{self, Ready},
    net::IpAddr,
    str::FromStr,
};

use crate::{
    announce::{
        error::{AnnounceError, Result},
        models::torrent::{Announce, AnnounceEvent},
    },
    services::announce_service::is_torrent_client_allowed,
    Tracker,
};
use actix_web::{
    dev,
    web::{Data, Path},
    FromRequest, HttpRequest, HttpResponse,
};
use arcadia_shared::tracker::models::{
    peer::{self, Peer},
    torrent_update::{self, TorrentUpdate},
    user::Passkey,
    user_update::{self, UserUpdate},
};
use chrono::{Duration, Utc};
use rand::{rng, seq::IteratorRandom, Rng};

#[derive(Debug)]
pub struct UserAgent(pub String);

impl std::ops::Deref for UserAgent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl FromRequest for UserAgent {
    type Error = AnnounceError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let agent = req
            .headers()
            .get("User-Agent")
            .ok_or(AnnounceError::NoUserAgent)
            .and_then(|s| {
                let agent_string = s.to_str()?;
                if agent_string.len() > 64 {
                    // Block user agent strings that are too long. (For Database reasons)
                    Err(AnnounceError::UserAgentTooLong)
                } else {
                    Ok(UserAgent(String::from(agent_string)))
                }
            });

        future::ready(agent)
    }
}

pub struct ClientIp(pub std::net::IpAddr);

impl FromRequest for ClientIp {
    type Error = AnnounceError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let tracker = req.app_data::<Data<Tracker>>().expect("app data set");

        let header_name_opt = &tracker.env.reverse_proxy_client_ip_header_name;

        let ip_result = if let Some(header_name) = header_name_opt {
            req.headers()
                .get(header_name)
                .and_then(|h| h.to_str().ok())
                .and_then(|s| {
                    s.split(',')
                        .next_back()
                        .and_then(|s| s.trim().parse::<IpAddr>().ok())
                })
                .map(ClientIp)
                .ok_or(AnnounceError::InternalTrackerError)
        } else {
            req.peer_addr()
                .map(|addr| ClientIp(addr.ip()))
                .ok_or(AnnounceError::InternalTrackerError)
        };

        future::ready(ip_result)
    }
}

#[utoipa::path(
    get,
    operation_id = "Announce",
    tag = "Announce",
    path = "/{passkey}/announce",
    responses(
        (status = 200, description = "Announce"),
    )
)]
pub async fn exec(
    arc: Data<Tracker>,
    passkey: Path<String>,
    // agent: UserAgent,
    ann: Announce,
    ClientIp(ip): ClientIp,
) -> Result<HttpResponse> {
    // let headers = req.headers();
    // if headers.contains_key(ACCEPT_LANGUAGE)
    //     || headers.contains_key(REFERER)
    //     || headers.contains_key(ACCEPT_CHARSET)
    //     // This header check may block Non-bittorrent client `Aria2` to access tracker,
    //     // Because they always add this header which other clients don't have.
    //     //
    //     // See: https://blog.rhilip.info/archives/1010/ ( in Chinese )
    //     || headers.contains_key("want-digest")
    // {
    //     return Err(AnnounceError::AbnormalAccess);
    // }

    if !is_torrent_client_allowed(&ann.peer_id, &arc.env.allowed_torrent_clients.clients) {
        return Err(AnnounceError::TorrentClientNotInWhitelist);
    }

    let passkey = Passkey::from_str(&passkey).or(Err(AnnounceError::InvalidPasskey))?;
    // Validate passkey
    let user_id = arc
        .passkey2id
        .read()
        .get(&passkey)
        .ok_or(AnnounceError::PasskeyNotFound)
        .cloned()
        .map_err(|_| AnnounceError::UserNotFound)?;

    // let user = arc
    //     .users
    //     .read()
    //     .get(&user_id)
    //     .ok_or(AnnounceError::UserNotFound)
    //     .cloned();

    // Validate torrent
    let torrent_id_res = arc
        .infohash2id
        .read()
        .get(&ann.info_hash)
        .ok_or(AnnounceError::InfoHashNotFound)
        .cloned();

    // if let Ok(user) = &user {
    //     if let Err(InfoHashNotFound) = torrent_id_res {
    //         tracker.unregistered_info_hash_updates.lock().upsert(
    //             unregistered_info_hash_update::Index {
    //                 user_id: user.id,
    //                 info_hash: queries.info_hash,
    //             },
    //             UnregisteredInfoHashUpdate {
    //                 created_at: now,
    //                 updated_at: now,
    //             },
    //         );
    //     }
    // }

    let torrent_id = torrent_id_res?;

    let now = Utc::now();

    let (
        upload_factor,
        download_factor,
        uploaded_delta,
        downloaded_delta,
        seeder_delta,
        leecher_delta,
        times_completed_delta,
        // is_visible,
        // is_active_after_stop,
        // user,
        user_id,
        has_requested_seed_list,
        has_requested_leech_list,
        response,
    ) = {
        let mut torrent_guard = arc.torrents.lock();
        let torrent = torrent_guard
            .get_mut(&torrent_id)
            .ok_or(AnnounceError::TorrentNotFound)?;

        if torrent.is_deleted {
            return Err(AnnounceError::TorrentIsDeleted);
        }

        // Change of upload/download compared to previous announce
        let uploaded_delta;
        let downloaded_delta;
        let seeder_delta;
        let leecher_delta;
        let times_completed_delta;
        // let is_visible;
        // let mut is_active_after_stop = false;

        if ann.event == AnnounceEvent::Stopped {
            // Try and remove the peer
            let removed_peer = torrent.peers.swap_remove(&peer::Index {
                user_id,
                peer_id: ann.peer_id,
            });
            // Check if peer was removed
            if let Some(peer) = removed_peer {
                // Calculate change in upload and download compared to previous
                // announce
                uploaded_delta = ann.uploaded.saturating_sub(peer.uploaded);
                downloaded_delta = ann.downloaded.saturating_sub(peer.downloaded);

                leecher_delta = 0 - peer.is_included_in_leech_list() as i32;
                seeder_delta = 0 - peer.is_included_in_seed_list() as i32;

                for (&index, &peer) in torrent.peers.iter() {
                    if index.user_id == user_id && peer.is_active {
                        // is_active_after_stop = true;

                        break;
                    }
                }
            } else {
                // Some clients (namely transmission) will keep sending
                // `stopped` events until a successful announce is received.
                // If a user's network is having issues, their peer might be
                // deleted for inactivity from missed announces. If their peer
                // isn't found when we receive a `stopped` event from them
                // after regaining network connectivity, we can't return an
                // error otherwise the client might enter into an infinite loop
                // of sending `stopped` events. To prevent this, we need to
                // send a warning (i.e. succcessful announce) instead, so that
                // the client can successfully restart its session.
                leecher_delta = 0;
                seeder_delta = 0;
                uploaded_delta = 0;
                downloaded_delta = 0;
            }

            times_completed_delta = 0;
            // is_visible = false;
        } else {
            // Insert the peer into the in-memory db
            let mut old_peer: Option<Peer> = None;
            let new_peer = *torrent
                .peers
                .entry(peer::Index {
                    user_id,
                    peer_id: ann.peer_id,
                })
                .and_modify(|peer| {
                    old_peer = Some(*peer);

                    peer.ip_address = ip;
                    peer.port = ann.port;
                    peer.is_seeder = ann.left == 0;
                    // peer.is_visible = peer.is_included_in_leech_list();
                    peer.is_active = true;
                    peer.has_sent_completed =
                        peer.has_sent_completed || ann.event == AnnounceEvent::Completed;
                    peer.updated_at = now;
                    peer.uploaded = ann.uploaded;
                    peer.downloaded = ann.downloaded;
                })
                .or_insert(peer::Peer {
                    ip_address: ip,
                    port: ann.port,
                    is_seeder: ann.left == 0,
                    is_active: true,
                    // is_visible: true,
                    has_sent_completed: ann.event == AnnounceEvent::Completed,
                    updated_at: now,
                    uploaded: ann.uploaded,
                    downloaded: ann.downloaded,
                });

            // is_visible = new_peer.is_visible;

            // Update the user and torrent seeding/leeching counts in the
            // in-memory db
            match old_peer {
                Some(old_peer) => {
                    leecher_delta = new_peer.is_included_in_leech_list() as i32
                        - old_peer.is_included_in_leech_list() as i32;
                    seeder_delta = new_peer.is_included_in_seed_list() as i32
                        - old_peer.is_included_in_seed_list() as i32;
                    times_completed_delta = (new_peer.is_seeder && !old_peer.is_seeder) as u32;

                    // Calculate change in upload and download compared to previous
                    // announce
                    if ann.uploaded < old_peer.uploaded || ann.downloaded < old_peer.downloaded {
                        // Client sent the same peer id but restarted the session
                        // Assume delta is 0
                        uploaded_delta = 0;
                        downloaded_delta = 0;
                    } else {
                        // Assume client continues previously tracked session
                        uploaded_delta = ann.uploaded - old_peer.uploaded;
                        downloaded_delta = ann.downloaded - old_peer.downloaded;
                    }

                    // Warn user if peer last announced less than
                    // announce_min_enforced seconds ago and it's
                    // not their first completed event
                    if old_peer
                        .updated_at
                        .checked_add_signed(Duration::seconds(arc.env.announce_min_enforced.into()))
                        .is_some_and(|blocked_until| blocked_until > now)
                        && (ann.event != AnnounceEvent::Completed || old_peer.has_sent_completed)
                    {
                        return Err(AnnounceError::RateLimitExceeded);
                    }
                }
                None => {
                    // new peer is inserted

                    // Make sure user is only allowed N peers per torrent.
                    let mut peer_count = 0;

                    for (&index, &peer) in torrent.peers.iter() {
                        if index.user_id == user_id && peer.is_active {
                            peer_count += 1;

                            if peer_count > arc.env.max_peers_per_torrent_per_user {
                                torrent.peers.swap_remove(&peer::Index {
                                    user_id,
                                    peer_id: ann.peer_id,
                                });

                                return Err(AnnounceError::PeersPerTorrentPerUserLimit(
                                    arc.env.max_peers_per_torrent_per_user,
                                ));
                            }
                        }
                    }

                    leecher_delta = new_peer.is_included_in_leech_list() as i32;
                    seeder_delta = new_peer.is_included_in_seed_list() as i32;
                    times_completed_delta = 0;

                    // Calculate change in upload and download compared to previous
                    // announce
                    uploaded_delta = 0;
                    downloaded_delta = 0;
                }
            }
        }

        // Has to be adjusted before the peer list is generated
        torrent.seeders = torrent.seeders.saturating_add_signed(seeder_delta);
        torrent.leechers = torrent.leechers.saturating_add_signed(leecher_delta);
        torrent.times_completed = torrent
            .times_completed
            .saturating_add(times_completed_delta);

        // Generate peer lists to return to client
        let mut peers_ipv4: Vec<u8> = Vec::new();
        let mut peers_ipv6: Vec<u8> = Vec::new();

        let mut has_requested_seed_list = false;
        let mut has_requested_leech_list = false;

        // Only provide peer list if
        // - it is not a stopped event,
        // - there exist leechers (we have to remember to update the torrent leecher count before this check)
        if ann.event != AnnounceEvent::Stopped && torrent.leechers > 0 {
            let mut peers: Vec<(&peer::Index, &Peer)> = Vec::with_capacity(std::cmp::min(
                ann.numwant,
                torrent.seeders as usize + torrent.leechers as usize,
            ));

            // Don't return peers with the same user id or those that are marked as inactive
            let valid_peers = torrent.peers.iter().filter(|(index, peer)| {
                index.user_id != user_id && peer.is_included_in_peer_list()
            });

            // Make sure leech peer lists are filled with seeds
            if ann.left > 0 && torrent.seeders > 0 && ann.numwant > peers.len() {
                has_requested_seed_list = true;
                peers.extend(
                    valid_peers
                        .clone()
                        .filter(|(_index, peer)| peer.is_seeder)
                        .choose_multiple(&mut rng(), ann.numwant),
                );
            }

            // Otherwise only send leeches until the numwant is reached
            if torrent.leechers > 0 && ann.numwant > peers.len() {
                has_requested_leech_list = true;
                peers.extend(
                    valid_peers
                        .filter(|(_index, peer)| !peer.is_seeder)
                        .choose_multiple(&mut rng(), ann.numwant.saturating_sub(peers.len())),
                );
            }

            // Split peers into ipv4 and ipv6 variants and serialize their socket
            // to bytes according to the bittorrent spec
            for (_index, peer) in peers.iter() {
                match peer.ip_address {
                    IpAddr::V4(ip) => {
                        peers_ipv4.extend(&ip.octets());
                        peers_ipv4.extend(&peer.port.to_be_bytes());
                    }
                    IpAddr::V6(ip) => {
                        peers_ipv6.extend(&ip.octets());
                        peers_ipv6.extend(&peer.port.to_be_bytes());
                    }
                }
            }
        }

        // Write out bencoded response (keys must be sorted to be within spec)
        let mut response: Vec<u8> = Vec::with_capacity(
            82 // literal characters
            + 5 * 5 // numbers with estimated digit quantity for each
            + peers_ipv4.len() * 6 + 5 // bytes per ipv4 plus estimated length prefix
            + peers_ipv6.len() * 18 + 5, // bytes per ipv6 plus estimated length prefix
        );

        response.extend(b"d8:completei");
        response.extend(torrent.seeders.to_string().as_bytes());
        response.extend(b"e10:downloadedi");
        response.extend(torrent.times_completed.to_string().as_bytes());
        response.extend(b"e10:incompletei");
        response.extend(torrent.leechers.to_string().as_bytes());

        response.extend(b"e8:intervali");
        let interval = rng().random_range(arc.env.announce_min..=arc.env.announce_max);
        response.extend(interval.to_string().as_bytes());
        response.extend(b"e12:min intervali");
        response.extend(arc.env.announce_min.to_string().as_bytes());
        response.extend(b"e5:peers");

        if peers_ipv4.is_empty() {
            response.extend(b"0:")
        } else {
            response.extend(peers_ipv4.len().to_string().as_bytes());
            response.extend(b":");
            response.extend(&peers_ipv4);
        }

        if !peers_ipv6.is_empty() {
            response.extend(b"6:peers6");
            response.extend(peers_ipv6.len().to_string().as_bytes());
            response.extend(b":");
            response.extend(peers_ipv6);
        }

        response.extend(b"e");

        let upload_factor = std::cmp::max(arc.env.global_upload_factor, torrent.upload_factor);
        let download_factor =
            std::cmp::min(arc.env.global_download_factor, torrent.download_factor);

        // Has to be dropped before any `await` calls.
        //
        // Unfortunately, `Drop` currently doesn't work in rust with borrowed values
        // so we have to use a giant scope instead.
        //
        // See:
        // - https://github.com/rust-lang/rust/issues/57478
        // - https://stackoverflow.com/questions/73519148/why-does-send-value-that-is-dropd-before-await-mean-the-future-is-send
        // - https://github.com/rust-lang/rust/issues/101135
        //    - Once this issue is fixed, we can remove the scope and rely solely on `Drop`.
        drop(torrent_guard);

        (
            upload_factor,
            download_factor,
            uploaded_delta,
            downloaded_delta,
            seeder_delta,
            leecher_delta,
            times_completed_delta,
            // is_visible,
            // is_active_after_stop,
            // user,
            user_id,
            has_requested_seed_list,
            has_requested_leech_list,
            response,
        )
    };

    let credited_uploaded_delta = upload_factor as u64 * uploaded_delta / 100;
    let credited_downloaded_delta = download_factor as u64 * downloaded_delta / 100;

    // let completed_at = if ann.event == AnnounceEvent::Completed {
    //     Some(now)
    // } else {
    //     None
    // };

    if seeder_delta != 0
        || leecher_delta != 0
        || has_requested_seed_list
        || has_requested_leech_list
    {
        arc.users.write().entry(user_id).and_modify(|user| {
            user.num_seeding = user.num_seeding.saturating_add_signed(seeder_delta);
            user.num_leeching = user.num_leeching.saturating_add_signed(leecher_delta);

            // TODO: setup seed/leech lists getting rate limiting
            // has been partially done in unit3d-announce
            // if has_requested_seed_list {
            //     user.receive_seed_list_rates.tick();
            // }

            // if has_requested_leech_list {
            //     user.receive_leech_list_rates.tick();
            // }
        });
    }

    if credited_uploaded_delta != 0 || credited_downloaded_delta != 0 {
        arc.user_updates.lock().upsert(
            user_update::Index { user_id },
            UserUpdate {
                uploaded_delta: credited_uploaded_delta,
                downloaded_delta: credited_downloaded_delta,
                real_uploaded_delta: ann.uploaded,
                real_downloaded_delta: ann.downloaded,
            },
        );
    }

    if seeder_delta != 0
        || leecher_delta != 0
        || times_completed_delta != 0
        || uploaded_delta != 0
        || downloaded_delta != 0
    {
        arc.torrent_updates.lock().upsert(
            torrent_update::Index { torrent_id },
            TorrentUpdate {
                seeder_delta,
                leecher_delta,
                times_completed_delta,
            },
        );
    }

    Ok(HttpResponse::Ok().body(response))
}
