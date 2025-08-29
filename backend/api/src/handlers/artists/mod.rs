pub mod create_artists;
pub mod get_artist_publications;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_artists::exec::<R>))
            .route(get().to(self::get_artist_publications::exec::<R>)),
    );
}
