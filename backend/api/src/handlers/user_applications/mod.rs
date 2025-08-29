pub mod create_user_application;
pub mod get_user_applications;
pub mod update_user_application_status;

use actix_web::web::{get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_user_application::exec::<R>))
            .route(get().to(self::get_user_applications::exec::<R>))
            .route(put().to(self::update_user_application_status::exec::<R>)),
    );
}
