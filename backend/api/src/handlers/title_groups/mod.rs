pub mod create_title_group;
pub mod create_title_group_comment;
pub mod edit_title_group;
pub mod get_title_group;
pub mod get_title_group_info_lite;

use actix_web::web::{get, post, put, resource, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_title_group::exec))
            .route(get().to(self::get_title_group::exec))
            .route(put().to(self::edit_title_group::exec)),
    );
    cfg.service(resource("/lite").route(post().to(self::get_title_group_info_lite::exec)));
    cfg.service(resource("/comments").route(post().to(self::create_title_group_comment::exec)));
}
