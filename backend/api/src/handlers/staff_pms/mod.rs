pub mod create_staff_pm;
pub mod create_staff_pm_message;
pub mod get_staff_pm;
pub mod list_staff_pms;
pub mod resolve_staff_pm;
pub mod unresolve_staff_pm;

use actix_web::web::{get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_staff_pm::exec::<R>))
            .route(get().to(self::list_staff_pms::exec::<R>)),
    );
    cfg.service(resource("/messages").route(post().to(self::create_staff_pm_message::exec::<R>)));
    cfg.service(resource("/{id}").route(get().to(self::get_staff_pm::exec::<R>)));
    cfg.service(resource("/{id}/resolve").route(put().to(self::resolve_staff_pm::exec::<R>)));
    cfg.service(resource("/{id}/unresolve").route(put().to(self::unresolve_staff_pm::exec::<R>)));
}
