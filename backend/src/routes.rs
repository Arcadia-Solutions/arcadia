use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::{
    announce_handler::handle_announce,
    artist_handler::{
        add_affiliated_artists, add_artist, get_artist_publications, get_artists_lite,
    },
    auth_handler::{login, register, validate_bearer_auth},
    edition_group_handler::add_edition_group,
    invitation_handler::send_invitation,
    master_group_handler::add_master_group,
    scrapers::{
        open_library::get_open_library_data,
        tmdb::{get_tmdb_movie_data, get_tmdb_tv_data},
    },
    series_handler::{add_series, get_series},
    subscriptions_handler::{add_subscription, remove_subscription},
    title_group_comment_handler::add_title_group_comment,
    title_group_handler::{add_title_group, get_title_group, get_title_group_info_lite},
    torrent_handler::{download_dottorrent_file, find_torrents, upload_torrent},
    torrent_report_handler::add_torrent_report,
    torrent_request_handler::add_torrent_request,
    torrent_request_vote_handler::add_torrent_request_vote,
    user_handler::{get_me, get_user},
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_announce).service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(validate_bearer_auth))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/user", web::get().to(get_user))
            .route("/me", web::get().to(get_me))
            .route("/invitation", web::post().to(send_invitation))
            .route("/master-group", web::post().to(add_master_group))
            .route("/title-group", web::post().to(add_title_group))
            .route("/title-group", web::get().to(get_title_group))
            .route(
                "/title-group/lite",
                web::get().to(get_title_group_info_lite),
            )
            .route("/edition-group", web::post().to(add_edition_group))
            .route("/torrent", web::post().to(upload_torrent))
            .route("/torrent", web::get().to(download_dottorrent_file))
            .route("/report/torrent", web::post().to(add_torrent_report))
            .route("/search/torrent", web::post().to(find_torrents))
            .route("/search/artist/lite", web::get().to(get_artists_lite))
            .route("/artist", web::post().to(add_artist))
            .route("/artist", web::get().to(get_artist_publications))
            .route(
                "/affiliated-artists",
                web::post().to(add_affiliated_artists),
            )
            .route(
                "/title-group-comment",
                web::post().to(add_title_group_comment),
            )
            .route("/series", web::post().to(add_series))
            .route("/series", web::get().to(get_series))
            .route("/torrent-request", web::post().to(add_torrent_request))
            .route(
                "/torrent-request/vote",
                web::post().to(add_torrent_request_vote),
            )
            .route("/subscription", web::post().to(add_subscription))
            .route("/subscription", web::delete().to(remove_subscription))
            .route(
                "/external_db/open_library",
                web::get().to(get_open_library_data),
            )
            .route(
                "/external_db/tmdb/movie",
                web::get().to(get_tmdb_movie_data),
            )
            .route("/external_db/tmdb/tv", web::get().to(get_tmdb_tv_data)),
    );
}
