pub mod create_edition_group;
pub mod edit_edition_group;
use actix_web::web::{post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_edition_group::exec::<R>))
            .route(put().to(self::edit_edition_group::exec::<R>)),
    );
}
