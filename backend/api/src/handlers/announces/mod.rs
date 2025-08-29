pub mod handle_announce;

use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/{passkey}").route(get().to(self::handle_announce::exec::<R>)));
}
