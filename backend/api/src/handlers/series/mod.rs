pub mod add_title_group;
pub mod create_series;
pub mod edit_series;
pub mod get_series;

use actix_web::web::{get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_series::exec::<R>))
            .route(get().to(self::get_series::exec::<R>))
            .route(put().to(self::edit_series::exec::<R>)),
    );
    cfg.service(resource("/title-group").route(post().to(self::add_title_group::exec::<R>)));
}
