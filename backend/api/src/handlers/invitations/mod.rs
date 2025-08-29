pub mod create_invitation;

use actix_web::web::{post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::create_invitation::exec::<R>)));
}
