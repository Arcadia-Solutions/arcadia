pub mod create_title_group;
pub mod create_title_group_comment;
pub mod delete_title_group;
pub mod edit_title_group;
pub mod edit_title_group_comment;
pub mod get_title_group;
pub mod get_title_group_info_lite;

use actix_web::web::{delete, get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_title_group::exec::<R>))
            .route(get().to(self::get_title_group::exec::<R>))
            .route(put().to(self::edit_title_group::exec::<R>))
            .route(delete().to(self::delete_title_group::exec::<R>)),
    );
    cfg.service(resource("/lite").route(post().to(self::get_title_group_info_lite::exec::<R>)));
    cfg.service(
        resource("/comments").route(post().to(self::create_title_group_comment::exec::<R>)),
    );
    cfg.service(
        resource("/comments/{id}").route(put().to(self::edit_title_group_comment::exec::<R>)),
    );
}
