pub mod rehash_torrents;

use actix_web::web::{post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/rehash-torrents").route(post().to(self::rehash_torrents::exec::<R>)));
}
