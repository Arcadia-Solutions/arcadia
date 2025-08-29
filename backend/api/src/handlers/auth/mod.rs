pub mod login;
pub mod refresh_token;
pub mod register;

use actix_web::web::{post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/register").route(post().to(self::register::exec::<R>)));
    cfg.service(resource("/login").route(post().to(self::login::exec::<R>)));
    cfg.service(resource("/refresh-token").route(post().to(self::refresh_token::exec::<R>)));
}
