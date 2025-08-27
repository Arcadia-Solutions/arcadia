pub mod create_affiliated_artists;
pub mod remove_affiliated_artists;

use actix_web::web::{delete, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_affiliated_artists::exec))
            .route(delete().to(self::remove_affiliated_artists::exec)),
    );
}
