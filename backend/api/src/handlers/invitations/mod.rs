pub mod create_invitation;
pub mod search_sent_invitations;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(get().to(self::search_sent_invitations::exec::<R>))
            .route(post().to(self::create_invitation::exec::<R>)),
    );
}
