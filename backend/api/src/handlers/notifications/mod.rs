pub mod get_notifications_forum_thread_posts;

use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/forum-thread-posts")
            .route(get().to(self::get_notifications_forum_thread_posts::exec::<R>)),
    );
}
