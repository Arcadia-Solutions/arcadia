pub mod search_logs;

use crate::middlewares::auth_middleware;
use actix_web::web::{get, resource, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(get().to(self::search_logs::exec::<R>))
            .wrap(HttpAuthentication::with_fn(
                auth_middleware::authenticate_user::<R>,
            )),
    );
}
