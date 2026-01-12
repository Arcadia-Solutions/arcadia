use actix_web::web::{self, put, resource, scope};

use crate::{
    announce::handlers::announce::config as AnnouncesConfig,
    handlers::{
        settings::update_settings,
        torrents::{update_torrent_factors, upsert_torrent},
        users::upsert_user,
    },
    middleware::authenticate_backend,
};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn init(cfg: &mut web::ServiceConfig) {
    // TODO: protect by only allowing requests from backend's ip
    cfg.service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(authenticate_backend))
            .service(resource("/torrents").route(put().to(upsert_torrent::exec)))
            .service(
                resource("/torrents/{id}/up-down-factors")
                    .route(put().to(update_torrent_factors::exec)),
            )
            .service(resource("/users").route(put().to(upsert_user::exec)))
            .service(resource("/settings").route(put().to(update_settings::exec))),
    );
    cfg.service(scope("{passkey}").configure(AnnouncesConfig));
}
