pub mod buy_promotion;
pub mod change_user_class;
pub mod create_api_key;
pub mod edit_user;
pub mod edit_user_permissions;
pub mod get_me;
pub mod get_user;
pub mod get_user_conversations;
pub mod get_user_permissions;
pub mod get_user_settings;
pub mod lock_user_class;
pub mod set_user_custom_title;
pub mod update_user_settings;
pub mod warn_user;

use actix_web::web::{get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(get().to(self::get_user::exec::<R>))
            .route(put().to(self::edit_user::exec::<R>)),
    );
    cfg.service(resource("/warn").route(post().to(self::warn_user::exec::<R>)));
    cfg.service(resource("/me").route(get().to(self::get_me::exec::<R>)));
    cfg.service(resource("/api-keys").route(post().to(self::create_api_key::exec::<R>)));
    cfg.service(resource("/buy-promotion").route(post().to(self::buy_promotion::exec::<R>)));
    cfg.service(
        resource("/conversations").route(get().to(self::get_user_conversations::exec::<R>)),
    );
    cfg.service(
        resource("/settings")
            .route(get().to(self::get_user_settings::exec::<R>))
            .route(put().to(self::update_user_settings::exec::<R>)),
    );
    cfg.service(
        resource("/{id}/permissions")
            .route(get().to(self::get_user_permissions::exec::<R>))
            .route(put().to(self::edit_user_permissions::exec::<R>)),
    );
    cfg.service(resource("/{id}/lock-class").route(put().to(self::lock_user_class::exec::<R>)));
    cfg.service(resource("/{id}/class").route(put().to(self::change_user_class::exec::<R>)));
    cfg.service(
        resource("/{id}/custom-title").route(put().to(self::set_user_custom_title::exec::<R>)),
    );
}
