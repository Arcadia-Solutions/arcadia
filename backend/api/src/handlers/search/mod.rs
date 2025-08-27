pub mod search_artists_lite;
pub mod search_forum_thread;
pub mod search_title_group_info_lite;
pub mod search_torrent_requests;
pub mod search_torrents;

use actix_web::web::{post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/title-groups/lite").route(post().to(self::search_title_group_info_lite::exec)),
    );
    cfg.service(resource("/torrents/lite").route(post().to(self::search_torrents::exec)));
    cfg.service(resource("/artists/lite").route(post().to(self::search_artists_lite::exec)));
    cfg.service(
        resource("/torrent-requests").route(post().to(self::search_torrent_requests::exec)),
    );
    cfg.service(resource("/forum/threads").route(post().to(self::search_forum_thread::exec)));
}
