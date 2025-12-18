pub mod create_donation;
pub mod delete_donation;
pub mod edit_donation;
pub mod get_donation_settings;
pub mod get_donation_stats;
pub mod get_donations;
pub mod update_donation_settings;

use actix_web::web::{delete, get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(get().to(self::get_donations::exec::<R>))
            .route(post().to(self::create_donation::exec::<R>)),
    );
    cfg.service(
        resource("/{id}")
            .route(put().to(self::edit_donation::exec::<R>))
            .route(delete().to(self::delete_donation::exec::<R>)),
    );
    cfg.service(
        resource("/settings")
            .route(get().to(self::get_donation_settings::exec::<R>))
            .route(put().to(self::update_donation_settings::exec::<R>)),
    );
    cfg.service(resource("/stats").route(get().to(self::get_donation_stats::exec::<R>)));
}
