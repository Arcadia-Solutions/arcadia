pub mod create_collage;
pub mod create_collage_entries;
pub mod get_collage;
pub mod get_collage_entries;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_collage::exec::<R>))
            .route(get().to(self::get_collage::exec::<R>)),
    );
    cfg.service(
        resource("/entries")
            .route(post().to(self::create_collage_entries::exec::<R>))
            .route(get().to(self::get_collage_entries::exec::<R>)),
    );
}
