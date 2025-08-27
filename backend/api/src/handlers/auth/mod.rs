pub mod login;
pub mod refresh_token;
pub mod register;

use actix_web::web::{post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("/register").route(post().to(self::register::exec)));
    cfg.service(resource("/login").route(post().to(self::login::exec)));
    cfg.service(resource("/refresh-token").route(post().to(self::refresh_token::exec)));
}
