pub mod create_subscription_forum_thread_posts;
pub mod create_subscription_title_group_torrents;
pub mod remove_subscription_forum_thread_posts;
pub mod remove_subscription_title_group_torrents;

use actix_web::web::{delete, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/forum-thread-posts")
            .route(post().to(self::create_subscription_forum_thread_posts::exec::<R>))
            .route(delete().to(self::remove_subscription_forum_thread_posts::exec::<R>)),
    );
    cfg.service(
        resource("/title-group-torrents")
            .route(post().to(self::create_subscription_title_group_torrents::exec::<R>))
            .route(delete().to(self::remove_subscription_title_group_torrents::exec::<R>)),
    );
}
