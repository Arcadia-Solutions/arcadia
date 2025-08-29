pub mod create_subscription;
pub mod remove_subscription;

use actix_web::web::{delete, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_subscription::exec::<R>))
            .route(delete().to(self::remove_subscription::exec::<R>)),
    );
}
