pub mod create_wiki_article;
pub mod get_wiki_article;

use actix_web::web::{get, post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/articles")
            .route(post().to(self::create_wiki_article::exec::<R>))
            .route(get().to(self::get_wiki_article::exec::<R>)),
    );
}
