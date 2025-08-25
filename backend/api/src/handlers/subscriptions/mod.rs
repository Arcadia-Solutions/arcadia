pub mod create_subscription;
pub mod remove_subscription;

use actix_web::web::{delete, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_subscription::exec))
            .route(delete().to(self::remove_subscription::exec)),
    );
}
