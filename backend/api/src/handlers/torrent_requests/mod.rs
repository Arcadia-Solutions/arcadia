pub mod create_torrent_request;
pub mod create_torrent_request_vote;
pub mod fill_torrent_request;
pub mod get_torrent_request;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_torrent_request::exec::<R>))
            .route(get().to(self::get_torrent_request::exec::<R>)),
    );
    cfg.service(resource("/fill").route(post().to(self::fill_torrent_request::exec::<R>)));
    cfg.service(resource("/vote").route(post().to(self::create_torrent_request_vote::exec::<R>)));
}
