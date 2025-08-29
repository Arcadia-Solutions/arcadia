pub mod create_edition_group;
use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::create_edition_group::exec::<R>)));
}
