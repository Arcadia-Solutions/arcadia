pub mod apply_tag;
pub mod create_tag;
pub mod remove_tag;

use actix_web::web::{delete, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::create_tag::exec::<R>)));
    cfg.service(resource("/apply").route(post().to(self::apply_tag::exec::<R>)));
    cfg.service(resource("/remove").route(delete().to(self::remove_tag::exec::<R>)));
}
