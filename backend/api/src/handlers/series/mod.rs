pub mod create_series;
pub mod get_series;

use actix_web::web::{get, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_series::exec))
            .route(get().to(self::get_series::exec)),
    );
}
