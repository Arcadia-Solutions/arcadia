pub mod get_comic_vine_data;
pub mod get_isbn_data;
pub mod get_musicbrainz_data;
pub mod get_tmdb_data;

use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/isbn").route(get().to(self::get_isbn_data::exec)));
    cfg.service(resource("/tmdb").route(get().to(self::get_tmdb_data::exec::<R>)));
    cfg.service(resource("/comic-vine").route(get().to(self::get_comic_vine_data::exec::<R>)));
    cfg.service(resource("/musicbrainz").route(get().to(self::get_musicbrainz_data::exec::<R>)));
}
