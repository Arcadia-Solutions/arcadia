pub mod create_gift;

use actix_web::web::{post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::create_gift::exec)));
}
