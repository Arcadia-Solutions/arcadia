pub mod create_api_key;
pub mod edit_user;
pub mod get_me;
pub mod get_registered_users;
pub mod get_user;
pub mod get_user_conversations;
pub mod warn_user;

use actix_web::web::{get, post, put, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(get().to(self::get_user::exec))
            .route(put().to(self::edit_user::exec)),
    );
    cfg.service(resource("/warnings").route(post().to(self::warn_user::exec)));
    cfg.service(resource("/me").route(get().to(self::get_me::exec)));
    cfg.service(resource("/registered").route(get().to(self::get_registered_users::exec)));
    cfg.service(resource("/api-keys").route(post().to(self::create_api_key::exec)));
    cfg.service(resource("/conversations").route(get().to(self::get_user_conversations::exec)));
}
