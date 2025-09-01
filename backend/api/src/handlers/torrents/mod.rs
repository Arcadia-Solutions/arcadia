pub mod create_torrent;
pub mod create_torrent_report;
pub mod delete_torrent;
pub mod download_dottorrent_file;
pub mod edit_torrent;
pub mod get_registered_torrents;
pub mod get_top_torrents;
pub mod get_upload_information;

use actix_web::web::{delete, get, post, put, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(post().to(self::create_torrent::exec::<R>))
            .route(get().to(self::download_dottorrent_file::exec::<R>))
            .route(put().to(self::edit_torrent::exec::<R>))
            .route(delete().to(self::delete_torrent::exec::<R>)),
    );
    cfg.service(resource("/registered").route(get().to(self::get_registered_torrents::exec::<R>)));
    cfg.service(resource("/upload-info").route(get().to(self::get_upload_information::exec::<R>)));
    cfg.service(resource("/top").route(get().to(self::get_top_torrents::exec::<R>)));
    cfg.service(resource("/reports").route(get().to(self::create_torrent_report::exec::<R>)));
}
