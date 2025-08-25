pub mod create_artists;
pub mod get_artist_publications;

use actix_web::web::{get, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_artists::exec))
            .route(get().to(self::get_artist_publications::exec)),
    );
}
