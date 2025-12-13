pub mod create_forum_category;
pub mod create_forum_post;
pub mod create_forum_sub_category;
pub mod create_forum_thread;
pub mod edit_forum_category;
pub mod edit_forum_post;
pub mod edit_forum_sub_category;
pub mod edit_forum_thread;
pub mod get_forum;
pub mod get_forum_sub_category_threads;
pub mod get_forum_thread;
pub mod get_forum_thread_posts;

use actix_web::web::{get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::get_forum::exec::<R>)));
    cfg.service(
        resource("/category")
            .route(post().to(self::create_forum_category::exec::<R>))
            .route(put().to(self::edit_forum_category::exec::<R>)),
    );
    cfg.service(
        resource("/thread")
            .route(get().to(self::get_forum_thread::exec::<R>))
            .route(put().to(self::edit_forum_thread::exec::<R>))
            .route(post().to(self::create_forum_thread::exec::<R>)),
    );
    cfg.service(resource("/thread/posts").route(get().to(self::get_forum_thread_posts::exec::<R>)));
    cfg.service(
        resource("/post")
            .route(post().to(self::create_forum_post::exec::<R>))
            .route(put().to(self::edit_forum_post::exec::<R>)),
    );
    cfg.service(
        resource("/sub-category")
            .route(get().to(self::get_forum_sub_category_threads::exec::<R>))
            .route(post().to(self::create_forum_sub_category::exec::<R>))
            .route(put().to(self::edit_forum_sub_category::exec::<R>)),
    );
}
