use actix_web::web::{self, scope};

use crate::announce::handlers::announce::config as AnnouncesConfig;
// use actix_web_httpauth::middleware::HttpAuthentication;

// use crate::middleware::authenticate_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("{passkey}").configure(AnnouncesConfig));
    // TODO: protect by only allowing requests from backend's ip
    // cfg.service(
    //          .wrap(HttpAuthentication::with_fn(authenticate_user(req, passkey))),
    // );
}
