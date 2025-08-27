pub mod create_conversation;
pub mod create_conversation_message;
pub mod get_conversation;

use actix_web::web::{get, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_conversation::exec))
            .route(get().to(self::get_conversation::exec)),
    );

    cfg.service(resource("/messages").route(post().to(self::create_conversation_message::exec)));
}
