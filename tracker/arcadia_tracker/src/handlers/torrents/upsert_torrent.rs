use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_shared::tracker::models::{
    peer,
    torrent::{APIInsertTorrent, Torrent},
};
use log::{debug, info};

use crate::Tracker;

pub async fn exec(arc: Data<Tracker>, torrent: Json<APIInsertTorrent>) -> HttpResponse {
    let mut torrents = arc.torrents.lock();

    if let Some(existing_torrent) = torrents.get_mut(&torrent.id) {
        info!("Updating torrent with id {}.", torrent.id);

        existing_torrent.is_deleted = torrent.is_deleted;
        existing_torrent.upload_factor = torrent.upload_factor as i16;
        existing_torrent.download_factor = torrent.download_factor as i16;

        debug!("Updated torrent: {:?}", torrent);
    } else {
        info!("Inserting torrent with id {}.", torrent.id);

        torrents.insert(
            torrent.id,
            Torrent {
                is_deleted: torrent.is_deleted,
                seeders: torrent.seeders,
                leechers: torrent.leechers,
                times_completed: torrent.times_completed,
                download_factor: torrent.download_factor as i16,
                upload_factor: torrent.upload_factor as i16,
                peers: peer::Map::new(),
            },
        );

        arc.infohash2id
            .write()
            .insert(torrent.info_hash, torrent.id);

        debug!("Inserted torrent: {:?}", torrent);
    }

    HttpResponse::Ok().finish()
}
