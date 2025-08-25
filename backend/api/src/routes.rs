use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::auth_handler::authenticate_user;
use crate::handlers::title_group_handler::edit_title_group;
use crate::handlers::torrent_request_handler::search_torrent_requests;
use crate::handlers::user_handler::add_api_key;
use crate::handlers::user_handler::get_registered_users;
use crate::handlers::{
    announce_handler::handle_announce,
    artist_handler::{
        add_affiliated_artists, add_artists, get_artist_publications, get_artists_lite,
        remove_affiliated_artists,
    },
    auth_handler::{login, refresh_token, register},
    conversation_handler::{
        add_conversation, add_conversation_message, get_conversation, get_user_conversations,
    },
    edition_group_handler::add_edition_group,
    forum_handler::{
        add_forum_post, add_forum_thread, get_forum, get_forum_sub_category_threads,
        get_forum_thread, search_forum_thread,
    },
    gift_handler::send_gift,
    home_handler::get_home,
    invitation_handler::send_invitation,
    master_group_handler::add_master_group,
    scrapers::{
        comic_vine::get_comic_vine_data, isbn::get_isbn_data, musicbrainz::get_musicbrainz_data,
        tmdb::get_tmdb_data,
    },
    series_handler::{add_series, get_series},
    subscriptions_handler::{add_subscription, remove_subscription},
    title_group_comment_handler::add_title_group_comment,
    title_group_handler::{
        add_title_group, get_title_group, get_title_group_info_lite, search_title_group_info_lite,
    },
    torrent_handler::{
        delete_torrent, download_dottorrent_file, edit_torrent, find_torrents,
        get_registered_torrents, get_top_torrents, get_upload_information, upload_torrent,
    },
    torrent_report_handler::add_torrent_report,
    torrent_request_handler::{add_torrent_request, fill_torrent_request, get_torrent_request},
    torrent_request_vote_handler::add_torrent_request_vote,
    user_application_handler::{
        add_user_application, get_user_applications, update_user_application_status,
    },
    user_handler::{edit_user, get_me, get_user, warn_user},
    wiki_handler::{add_wiki_article, get_wiki_article},
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(handle_announce).service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/apply", web::post().to(add_user_application))
            .route("/api-key", web::post().to(add_api_key))
            .route("/user-application", web::get().to(get_user_applications))
            .route(
                "/user-application",
                web::put().to(update_user_application_status),
            )
            .route("/refresh-token", web::post().to(refresh_token))
            .route("/home", web::get().to(get_home))
            .route("/user", web::get().to(get_user))
            .route("/user", web::put().to(edit_user))
            .route("/user/warn", web::post().to(warn_user))
            .route("/me", web::get().to(get_me))
            .route("/invitation", web::post().to(send_invitation))
            .route("/master-group", web::post().to(add_master_group))
            .route("/title-group", web::post().to(add_title_group))
            .route("/title-group", web::get().to(get_title_group))
            .route("/title-group", web::put().to(edit_title_group))
            .route(
                "/title-group/lite",
                web::get().to(get_title_group_info_lite),
            )
            .route(
                "/search/title-group/lite",
                web::get().to(search_title_group_info_lite),
            )
            .route("/edition-group", web::post().to(add_edition_group))
            .route("/torrent", web::post().to(upload_torrent))
            .route("/torrent", web::put().to(edit_torrent))
            .route("/torrent", web::get().to(download_dottorrent_file))
            .route(
                "/registered-torrents",
                web::get().to(get_registered_torrents),
            )
            .route("/registered-users", web::get().to(get_registered_users))
            .route("/upload", web::get().to(get_upload_information))
            .route("/torrent", web::delete().to(delete_torrent))
            .route("/torrent/top", web::get().to(get_top_torrents))
            .route("/report/torrent", web::post().to(add_torrent_report))
            .route("/search/torrent/lite", web::post().to(find_torrents))
            .route("/search/artist/lite", web::get().to(get_artists_lite))
            .route("/artists", web::post().to(add_artists))
            .route("/artist", web::get().to(get_artist_publications))
            .route(
                "/affiliated-artists",
                web::post().to(add_affiliated_artists),
            )
            .route(
                "/affiliated-artists",
                web::delete().to(remove_affiliated_artists),
            )
            .route(
                "/title-group-comment",
                web::post().to(add_title_group_comment),
            )
            .route("/series", web::post().to(add_series))
            .route("/series", web::get().to(get_series))
            .route("/torrent-request", web::post().to(add_torrent_request))
            .route("/torrent-request", web::get().to(get_torrent_request))
            .route(
                "/torrent-request/fill",
                web::post().to(fill_torrent_request),
            )
            .route(
                "/torrent-request/vote",
                web::post().to(add_torrent_request_vote),
            )
            .route(
                "/search/torrent-request",
                web::get().to(search_torrent_requests),
            )
            .route("/subscription", web::post().to(add_subscription))
            .route("/subscription", web::delete().to(remove_subscription))
            .route("/gift", web::post().to(send_gift))
            .route("/forum", web::get().to(get_forum))
            .route(
                "/forum/sub-category",
                web::get().to(get_forum_sub_category_threads),
            )
            .route("/forum/thread", web::get().to(get_forum_thread))
            .route("/search/forum/thread", web::get().to(search_forum_thread))
            .route("/forum/thread", web::post().to(add_forum_thread))
            .route("/forum/post", web::post().to(add_forum_post))
            .route("/wiki/article", web::post().to(add_wiki_article))
            .route("/wiki/article", web::get().to(get_wiki_article))
            .route("/conversation", web::post().to(add_conversation))
            .route("/conversation", web::get().to(get_conversation))
            .route("/conversations", web::get().to(get_user_conversations))
            .route(
                "/conversation/message",
                web::post().to(add_conversation_message),
            )
            .route("/external_db/isbn", web::get().to(get_isbn_data))
            .route("/external_db/tmdb", web::get().to(get_tmdb_data))
            .route("/external_db/isbn", web::get().to(get_isbn_data))
            .route(
                "/external_db/comic_vine",
                web::get().to(get_comic_vine_data),
            )
            .route(
                "/external_db/musicbrainz",
                web::get().to(get_musicbrainz_data),
            ),
    );
}
