pub mod create_edition_group;
use actix_web::web::{get, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::create_edition_group::exec)));
}
