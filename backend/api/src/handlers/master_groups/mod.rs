pub mod create_master_group;

use actix_web::web::{post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::create_master_group::exec)));
}
