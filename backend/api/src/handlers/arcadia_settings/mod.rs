pub mod get_arcadia_settings;
pub mod update_arcadia_settings;

use actix_web::web::{get, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(get().to(self::get_arcadia_settings::exec::<R>))
            .route(put().to(self::update_arcadia_settings::exec::<R>)),
    );
}
