pub mod get_notification_counts;
pub mod get_notifications;
pub mod mark_torrent_deletions_as_read;
pub mod notification_stream;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(get().to(self::get_notifications::exec::<R>)))
        .service(resource("/counts").route(get().to(self::get_notification_counts::exec::<R>)))
        .service(resource("/stream").route(get().to(self::notification_stream::exec::<R>)))
        .service(
            resource("/torrent-deletions/read")
                .route(post().to(self::mark_torrent_deletions_as_read::exec::<R>)),
        );
}
