pub mod get_forum_stats;
pub mod get_torrent_stats;

use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/torrents").route(get().to(self::get_torrent_stats::exec::<R>)))
        .service(resource("/forum").route(get().to(self::get_forum_stats::exec::<R>)));
}
