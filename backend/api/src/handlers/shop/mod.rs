pub mod buy_freeleech_tokens;
pub mod buy_promotion;
pub mod buy_upload;
pub mod get_pricing;
pub mod get_purchase_history;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/pricing").route(get().to(self::get_pricing::exec::<R>)));
    cfg.service(resource("/history").route(get().to(self::get_purchase_history::exec::<R>)));
    cfg.service(resource("/buy-promotion").route(post().to(self::buy_promotion::exec::<R>)));
    cfg.service(resource("/buy-upload").route(post().to(self::buy_upload::exec::<R>)));
    cfg.service(
        resource("/buy-freeleech-tokens").route(post().to(self::buy_freeleech_tokens::exec::<R>)),
    );
}
