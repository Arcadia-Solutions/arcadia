use actix_web::web::{self};
// use actix_web_httpauth::middleware::HttpAuthentication;

// use crate::middleware::authenticate_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    // TODO: protect by only allowing requests from backend's ip
    cfg.service(
        web::scope("/{passkey}"), //.wrap(HttpAuthentication::with_fn(authenticate_user(req, passkey))),
    );
}
