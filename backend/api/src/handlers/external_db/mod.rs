pub mod get_comic_vine_data;
pub mod get_external_source_data;
pub mod get_isbn_data;
pub mod get_musicbrainz_data;
pub mod get_plugin_data;
pub mod get_tmdb_data;

use actix_web::web::{get, resource, ServiceConfig};
use arcadia_storage::{models::title_group::ContentType, redis::RedisPoolInterface};

use crate::handlers::scrapers::ExternalSource;

/// The external sources bundled with arcadia, as `(id, placeholder, content_types)`.
/// Each id is dispatched to its handler in `get_external_source_data`.
const BUILT_IN_EXTERNAL_SOURCES: &[(&str, &str, &[ContentType])] = &[
    ("isbn", "ISBN", &[ContentType::Book]),
    (
        "tmdb",
        "TMDB url",
        &[ContentType::Movie, ContentType::TVShow],
    ),
    ("comic-vine", "Comic Vine url", &[ContentType::Book]),
    ("musicbrainz", "MusicBrainz url", &[ContentType::Music]),
];

pub fn built_in_external_sources() -> Vec<ExternalSource> {
    BUILT_IN_EXTERNAL_SOURCES
        .iter()
        .map(|(id, placeholder, content_types)| ExternalSource {
            id: id.to_string(),
            placeholder: placeholder.to_string(),
            content_types: content_types.to_vec(),
        })
        .collect()
}

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    // a single route serves every external source, built in ones as well as plugins
    cfg.service(
        resource("/{source_id}").route(get().to(self::get_external_source_data::exec::<R>)),
    );
}
