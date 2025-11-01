use arcadia_storage::models::collage::SearchCollagesQuery;
use arcadia_storage::models::series::SearchSeriesQuery;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::handlers::{
    search::search_torrent_requests::SearchTorrentRequestsQuery,
    user_applications::get_user_applications::GetUserApplicationsQuery,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-backend API",),
    modifiers(&SecurityAddon),
    paths(
        crate::handlers::auth::register::exec,
        crate::handlers::auth::login::exec,
        crate::handlers::auth::refresh_token::exec,
        crate::handlers::users::get_user::exec,
        crate::handlers::users::edit_user::exec,
        crate::handlers::users::warn_user::exec,
        crate::handlers::users::get_user_conversations::exec,
        crate::handlers::users::get_me::exec,
        crate::handlers::users::get_registered_users::exec,
        crate::handlers::auth::create_user_application::exec,
        crate::handlers::user_applications::get_user_applications::exec,
        crate::handlers::user_applications::update_user_application_status::exec,
        crate::handlers::home::get_home::exec,
        crate::handlers::artists::get_artist_publications::exec,
        crate::handlers::artists::create_artists::exec,
        crate::handlers::affiliated_artists::create_affiliated_artists::exec,
        crate::handlers::affiliated_artists::remove_affiliated_artists::exec,
        crate::handlers::torrents::download_dottorrent_file::exec,
        crate::handlers::torrents::create_torrent::exec,
        crate::handlers::torrents::edit_torrent::exec,
        crate::handlers::torrents::get_registered_torrents::exec,
        crate::handlers::torrents::get_upload_information::exec,
        crate::handlers::torrents::get_top_torrents::exec,
        crate::handlers::torrents::delete_torrent::exec,
        crate::handlers::torrents::create_torrent_report::exec,
        crate::handlers::edition_groups::create_edition_group::exec,
        crate::handlers::invitations::create_invitation::exec,
        crate::handlers::master_groups::create_master_group::exec,
        crate::handlers::series::create_series::exec,
        crate::handlers::series::get_series::exec,
        crate::handlers::subscriptions::create_subscription::exec,
        crate::handlers::subscriptions::remove_subscription::exec,
        crate::handlers::title_groups::create_title_group_comment::exec,
        crate::handlers::title_groups::create_title_group::exec,
        crate::handlers::title_groups::edit_title_group::exec,
        crate::handlers::title_groups::get_title_group::exec,
        crate::handlers::title_groups::get_title_group_info_lite::exec,
        crate::handlers::search::search_torrents::exec,
        crate::handlers::search::search_title_group_info_lite::exec,
        crate::handlers::search::search_torrent_requests::exec,
        crate::handlers::search::search_artists_lite::exec,
        crate::handlers::search::search_collages::exec,
        crate::handlers::search::search_series::exec,
        crate::handlers::torrent_requests::create_torrent_request::exec,
        crate::handlers::torrent_requests::get_torrent_request::exec,
        crate::handlers::torrent_requests::fill_torrent_request::exec,
        crate::handlers::torrent_requests::create_torrent_request_vote::exec,
        crate::handlers::torrent_requests::create_torrent_request_comment::exec,
        crate::handlers::gifts::create_gift::exec,
        crate::handlers::forum::get_forum::exec,
        crate::handlers::forum::get_forum_sub_category_threads::exec,
        crate::handlers::forum::get_forum_thread::exec,
        crate::handlers::forum::get_forum_thread_posts::exec,
        crate::handlers::forum::create_forum_thread::exec,
        crate::handlers::forum::create_forum_post::exec,
        crate::handlers::wiki::create_wiki_article::exec,
        crate::handlers::wiki::get_wiki_article::exec,
        crate::handlers::conversations::create_conversation::exec,
        crate::handlers::conversations::get_conversation::exec,
        crate::handlers::conversations::create_conversation_message::exec,
        crate::handlers::staff_pms::create_staff_pm::exec,
        crate::handlers::staff_pms::create_staff_pm_message::exec,
        crate::handlers::staff_pms::get_staff_pm::exec,
        crate::handlers::staff_pms::list_staff_pms::exec,
        crate::handlers::staff_pms::resolve_staff_pm::exec,
        crate::handlers::collages::create_collage::exec,
        crate::handlers::collages::create_collage_entries::exec,
        crate::handlers::collages::get_collage::exec,
        crate::handlers::external_db::get_isbn_data::exec,
        crate::handlers::external_db::get_musicbrainz_data::exec,
        crate::handlers::external_db::get_tmdb_data::exec,
        crate::handlers::external_db::get_comic_vine_data::exec,
    ),
    components(schemas(
        GetUserApplicationsQuery,
        SearchTorrentRequestsQuery,
        SearchCollagesQuery,
        SearchSeriesQuery
    ),)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "http",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
