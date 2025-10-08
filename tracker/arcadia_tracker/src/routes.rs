use actix_web::web::{self, scope};
// use actix_web_httpauth::middleware::HttpAuthentication;

// use crate::middleware::authenticate_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{passkey}"), //.wrap(HttpAuthentication::with_fn(authenticate_user(req, passkey))),
    );
}
