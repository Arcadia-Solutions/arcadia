pub mod create_wiki_article;
pub mod get_wiki_article;

use actix_web::web::{get, post, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("/articles")
            .route(post().to(self::create_wiki_article::exec))
            .route(get().to(self::get_wiki_article::exec)),
    );
}
