pub mod create_forum_post;
pub mod create_forum_thread;
pub mod get_forum;
pub mod get_forum_sub_category_threads;
pub mod get_forum_thread;

use actix_web::web::{get, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::get_forum::exec)));
    cfg.service(
        resource("/thread")
            .route(get().to(self::get_forum_thread::exec))
            .route(post().to(self::create_forum_thread::exec)),
    );
    cfg.service(resource("/post").route(get().to(self::create_forum_post::exec)));
    cfg.service(
        resource("/sub-category").route(get().to(self::get_forum_sub_category_threads::exec)),
    );
}
