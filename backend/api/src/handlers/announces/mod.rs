pub mod handle_announce;

use actix_web::web::{get, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("/{passkey}").route(get().to(self::handle_announce::exec)));
}
