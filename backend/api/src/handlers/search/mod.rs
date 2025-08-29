pub mod search_artists_lite;
pub mod search_forum_thread;
pub mod search_title_group_info_lite;
pub mod search_torrent_requests;
pub mod search_torrents;

use actix_web::web::{post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/title-groups/lite")
            .route(post().to(self::search_title_group_info_lite::exec::<R>)),
    );
    cfg.service(resource("/torrents/lite").route(post().to(self::search_torrents::exec::<R>)));
    cfg.service(resource("/artists/lite").route(post().to(self::search_artists_lite::exec::<R>)));
    cfg.service(
        resource("/torrent-requests").route(post().to(self::search_torrent_requests::exec::<R>)),
    );
    cfg.service(resource("/forum/threads").route(post().to(self::search_forum_thread::exec::<R>)));
}
