pub mod search_artists_lite;
pub mod search_collages;
pub mod search_collages_lite;
pub mod search_forum;
pub mod search_series;
pub mod search_title_group_info_lite;
pub mod search_torrent_requests;
pub mod search_torrents;

use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/title-groups/lite")
            .route(get().to(self::search_title_group_info_lite::exec::<R>)),
    );
    cfg.service(resource("/torrents/lite").route(get().to(self::search_torrents::exec::<R>)));
    cfg.service(resource("/artists/lite").route(get().to(self::search_artists_lite::exec::<R>)));
    cfg.service(
        resource("/torrent-requests").route(get().to(self::search_torrent_requests::exec::<R>)),
    );
    cfg.service(resource("/collages").route(get().to(self::search_collages::exec::<R>)));
    cfg.service(resource("/collages/lite").route(get().to(self::search_collages_lite::exec::<R>)));
    cfg.service(resource("/series").route(get().to(self::search_series::exec::<R>)));
    cfg.service(resource("/forum").route(get().to(self::search_forum::exec::<R>)));
}
