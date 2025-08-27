pub mod get_home;

use actix_web::web::{get, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::get_home::exec)));
}
