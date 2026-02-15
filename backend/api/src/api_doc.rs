use crate::handlers::artists::delete_artist::DeleteArtistQuery;
use crate::handlers::title_groups::delete_title_group::DeleteTitleGroupQuery;
use arcadia_storage::models::artist::SearchArtistsQuery;
use arcadia_storage::models::collage::{
    DeleteCollageEntriesQuery, DeleteCollageQuery, SearchCollagesLiteQuery,
};
use arcadia_storage::models::donation::{
    DeletedDonation, DonationOrderBy, EditedDonation, SearchDonationsQuery, UserCreatedDonation,
};
use arcadia_storage::models::forum::{
    DeleteForumCategoryQuery, DeleteForumPostQuery, DeleteForumSubCategoryQuery,
    DeleteForumThreadQuery, ForumSearchQuery, ForumSubCategoryAllowedPoster,
    GetForumSubCategoryAllowedPostersQuery,
};
use arcadia_storage::models::invitation::{
    InvitationSearchOrderByColumn, SearchSentInvitationsQuery,
};
use arcadia_storage::models::series::SearchSeriesQuery;
use arcadia_storage::models::title_group_comment::TitleGroupCommentSearchQuery;
use arcadia_storage::models::title_group_tag::SearchTitleGroupTagsQuery;
use arcadia_storage::models::torrent::TorrentSearch;
use arcadia_storage::models::torrent_activity::{
    TorrentActivitiesOverview, TorrentActivity, TorrentActivityAndTitleGroup,
    TorrentActivityOrderByColumn,
};
use arcadia_storage::models::torrent_report::DeleteTorrentReportQuery;
use arcadia_storage::models::user::{SearchUsersQuery, UserSearchOrderBy};
use arcadia_storage::models::{collage::SearchCollagesQuery, forum::GetForumThreadPostsQuery};
use utoipa::{
    openapi::{
        schema::{Array, Object, Ref, Schema},
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        ContentBuilder, RefOr, ResponseBuilder,
    },
    Modify, OpenApi, PartialSchema,
};

use arcadia_storage::models::shop::{
    BuyFreeleechTokensRequest, BuyUploadRequest, FreeleechTokenDiscountTier,
    FreeleechTokensPriceCalculation, PromotionPricing, ShopItem, ShopPricing, ShopPurchase,
    UploadDiscountTier, UploadPriceCalculation,
};
use arcadia_storage::models::unauthorized_access::SearchUnauthorizedAccessQuery;
use arcadia_storage::models::user_edit_change_log::SearchUserEditChangeLogsQuery;

use crate::handlers::search::search_title_group_tags_lite::SearchTitleGroupTagsLiteQuery;
use crate::handlers::user_applications::get_user_applications::GetUserApplicationsQuery;
use crate::handlers::users::get_user_torrent_activities_overview::SeedersPerTorrent;
use crate::middlewares::side_effects::SideEffect;
use arcadia_storage::models::torrent_request::{
    SearchTorrentRequestsQuery, TorrentRequestSearchOrderBy,
};
use arcadia_storage::models::torrent_stats::{
    StatsInterval, TorrentStatsDataPoint, TorrentStatsGroupBy, TorrentStatsResponse,
};
use arcadia_storage::models::user_application::UserApplicationHierarchy;

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-backend API"),
    modifiers(&SecurityAddon, &SideEffectsResponseModifier),
    paths(
        crate::handlers::auth::register::exec,
        crate::handlers::auth::login::exec,
        crate::handlers::auth::logout::exec,
        crate::handlers::auth::refresh_token::exec,
        crate::handlers::users::get_user::exec,
        crate::handlers::users::edit_user::exec,
        crate::handlers::users::warn_user::exec,
        crate::handlers::users::get_user_conversations::exec,
        crate::handlers::users::get_me::exec,
        crate::handlers::users::get_user_settings::exec,
        crate::handlers::users::update_user_settings::exec,
        crate::handlers::users::change_user_class::exec,
        crate::handlers::users::edit_user_permissions::exec,
        crate::handlers::users::get_user_permissions::exec,
        crate::handlers::users::lock_user_class::exec,
        crate::handlers::users::set_user_custom_title::exec,
        crate::handlers::users::get_user_torrent_activities::exec,
        crate::handlers::users::get_user_torrent_activities_overview::exec,
        crate::handlers::shop::buy_promotion::exec,
        crate::handlers::shop::buy_upload::exec,
        crate::handlers::shop::buy_freeleech_tokens::exec,
        crate::handlers::shop::get_pricing::exec,
        crate::handlers::shop::get_purchase_history::exec,
        crate::handlers::auth::create_user_application::exec,
        crate::handlers::user_applications::get_user_applications::exec,
        crate::handlers::user_applications::update_user_application_status::exec,
        crate::handlers::unauthorized_access::search::exec,
        crate::handlers::user_edit_change_logs::search::exec,
        crate::handlers::home::get_home::exec,
        crate::handlers::artists::get_artist_publications::exec,
        crate::handlers::artists::create_artists::exec,
        crate::handlers::artists::edit_artist::exec,
        crate::handlers::artists::delete_artist::exec,
        crate::handlers::affiliated_artists::create_affiliated_artists::exec,
        crate::handlers::affiliated_artists::remove_affiliated_artists::exec,
        crate::handlers::torrents::download_dottorrent_file::exec,
        crate::handlers::torrents::create_torrent::exec,
        crate::handlers::torrents::edit_torrent::exec,
        crate::handlers::torrents::get_upload_information::exec,
        crate::handlers::torrents::get_top_torrents::exec,
        crate::handlers::torrents::delete_torrent::exec,
        crate::handlers::torrents::create_torrent_report::exec,
        crate::handlers::torrents::delete_torrent_report::exec,
        crate::handlers::torrents::set_torrent_staff_checked::exec,
        crate::handlers::torrents::get_torrent_peers::exec,
        crate::handlers::torrents::get_torrent_title_group::exec,
        crate::handlers::torrents::edit_torrent_up_down_factors::exec,
        crate::handlers::edition_groups::create_edition_group::exec,
        crate::handlers::edition_groups::edit_edition_group::exec,
        crate::handlers::invitations::create_invitation::exec,
        crate::handlers::invitations::search_sent_invitations::exec,
        crate::handlers::master_groups::create_master_group::exec,
        crate::handlers::series::create_series::exec,
        crate::handlers::series::get_series::exec,
        crate::handlers::series::get_series_entries::exec,
        crate::handlers::series::edit_series::exec,
        crate::handlers::series::delete_series::exec,
        crate::handlers::series::add_title_group::exec,
        crate::handlers::subscriptions::create_subscription_forum_thread_posts::exec,
        crate::handlers::subscriptions::remove_subscription_forum_thread_posts::exec,
        crate::handlers::subscriptions::create_subscription_title_group_torrents::exec,
        crate::handlers::subscriptions::remove_subscription_title_group_torrents::exec,
        crate::handlers::subscriptions::create_subscription_title_group_comments::exec,
        crate::handlers::subscriptions::remove_subscription_title_group_comments::exec,
        crate::handlers::notifications::get_notifications_forum_thread_posts::exec,
        crate::handlers::notifications::get_notifications_title_group_comments::exec,
        crate::handlers::notifications::get_notifications_staff_pm_messages::exec,
        crate::handlers::title_groups::create_title_group_comment::exec,
        crate::handlers::title_groups::edit_title_group_comment::exec,
        crate::handlers::title_groups::create_title_group::exec,
        crate::handlers::title_groups::edit_title_group::exec,
        crate::handlers::title_groups::get_title_group::exec,
        crate::handlers::title_groups::get_title_group_info_lite::exec,
        crate::handlers::title_groups::delete_title_group::exec,
        crate::handlers::title_group_tags::create_tag::exec,
        crate::handlers::title_group_tags::apply_tag::exec,
        crate::handlers::title_group_tags::remove_tag::exec,
        crate::handlers::title_group_tags::delete_tag::exec,
        crate::handlers::title_group_tags::edit_tag::exec,
        crate::handlers::search::search_title_group_tags::exec,
        crate::handlers::search::search_title_group_tags_lite::exec,
        crate::handlers::search::search_torrents::exec,
        crate::handlers::search::search_title_group_info_lite::exec,
        crate::handlers::search::search_torrent_requests::exec,
        crate::handlers::search::search_artists::exec,
        crate::handlers::search::search_artists_lite::exec,
        crate::handlers::search::search_collages::exec,
        crate::handlers::search::search_collages_lite::exec,
        crate::handlers::search::search_series::exec,
        crate::handlers::search::search_series_lite::exec,
        crate::handlers::search::search_forum::exec,
        crate::handlers::search::search_title_group_comments::exec,
        crate::handlers::search::search_users::exec,
        crate::handlers::search::search_users_lite::exec,
        crate::handlers::torrent_requests::create_torrent_request::exec,
        crate::handlers::torrent_requests::get_torrent_request::exec,
        crate::handlers::torrent_requests::fill_torrent_request::exec,
        crate::handlers::torrent_requests::create_torrent_request_vote::exec,
        crate::handlers::torrent_requests::create_torrent_request_comment::exec,
        crate::handlers::gifts::create_gift::exec,
        crate::handlers::donations::search_donations::exec,
        crate::handlers::donations::create_donation::exec,
        crate::handlers::donations::edit_donation::exec,
        crate::handlers::donations::delete_donation::exec,
        crate::handlers::forum::get_forum::exec,
        crate::handlers::forum::create_forum_category::exec,
        crate::handlers::forum::edit_forum_category::exec,
        crate::handlers::forum::edit_forum_sub_category::exec,
        crate::handlers::forum::create_forum_sub_category::exec,
        crate::handlers::forum::get_forum_sub_category_threads::exec,
        crate::handlers::forum::get_forum_thread::exec,
        crate::handlers::forum::get_forum_thread_posts::exec,
        crate::handlers::forum::create_forum_thread::exec,
        crate::handlers::forum::edit_forum_thread::exec,
        crate::handlers::forum::pin_forum_thread::exec,
        crate::handlers::forum::create_forum_post::exec,
        crate::handlers::forum::edit_forum_post::exec,
        crate::handlers::forum::delete_forum_category::exec,
        crate::handlers::forum::delete_forum_sub_category::exec,
        crate::handlers::forum::delete_forum_thread::exec,
        crate::handlers::forum::delete_forum_post::exec,
        crate::handlers::forum::get_forum_sub_category_allowed_posters::exec,
        crate::handlers::forum::add_forum_sub_category_allowed_poster::exec,
        crate::handlers::forum::remove_forum_sub_category_allowed_poster::exec,
        crate::handlers::wiki::create_wiki_article::exec,
        crate::handlers::wiki::get_wiki_article::exec,
        crate::handlers::wiki::edit_wiki_article::exec,
        crate::handlers::conversations::create_conversation::exec,
        crate::handlers::conversations::get_conversation::exec,
        crate::handlers::conversations::create_conversation_message::exec,
        crate::handlers::staff_pms::create_staff_pm::exec,
        crate::handlers::staff_pms::create_staff_pm_message::exec,
        crate::handlers::staff_pms::get_staff_pm::exec,
        crate::handlers::staff_pms::list_staff_pms::exec,
        crate::handlers::staff_pms::resolve_staff_pm::exec,
        crate::handlers::staff_pms::unresolve_staff_pm::exec,
        crate::handlers::collages::create_collage::exec,
        crate::handlers::collages::create_collage_entries::exec,
        crate::handlers::collages::get_collage::exec,
        crate::handlers::collages::get_collage_entries::exec,
        crate::handlers::collages::edit_collage::exec,
        crate::handlers::collages::delete_collage::exec,
        crate::handlers::collages::delete_collage_entries::exec,
        crate::handlers::css_sheets::create_css_sheet::exec,
        crate::handlers::css_sheets::edit_css_sheet::exec,
        crate::handlers::css_sheets::get_css_sheet_content::exec,
        crate::handlers::css_sheets::get_css_sheets::exec,
        crate::handlers::css_sheets::get_css_sheet::exec,
        crate::handlers::arcadia_settings::get_arcadia_settings::exec,
        crate::handlers::arcadia_settings::get_public_arcadia_settings::exec,
        crate::handlers::arcadia_settings::update_arcadia_settings::exec,
        crate::handlers::external_db::get_isbn_data::exec,
        crate::handlers::external_db::get_musicbrainz_data::exec,
        crate::handlers::external_db::get_tmdb_data::exec,
        crate::handlers::external_db::get_comic_vine_data::exec,
        crate::handlers::title_group_bookmarks::create_title_group_bookmark::exec,
        crate::handlers::title_group_bookmarks::edit_title_group_bookmark::exec,
        crate::handlers::title_group_bookmarks::get_title_group_bookmark::exec,
        crate::handlers::title_group_bookmarks::remove_title_group_bookmark::exec,
        crate::handlers::user_classes::create_user_class::exec,
        crate::handlers::user_classes::edit_user_class::exec,
        crate::handlers::user_classes::delete_user_class::exec,
        crate::handlers::user_classes::get_user_classes::exec,
        crate::handlers::stats::get_torrent_stats::exec,
    ),
    components(schemas(
        GetUserApplicationsQuery,
        UserApplicationHierarchy,
        SearchUnauthorizedAccessQuery,
        SearchUserEditChangeLogsQuery,
        SearchTorrentRequestsQuery,
        TorrentRequestSearchOrderBy,
        SearchArtistsQuery,
        DeleteArtistQuery,
        DeleteTitleGroupQuery,
        SearchCollagesQuery,
        SearchCollagesLiteQuery,
        DeleteCollageQuery,
        DeleteCollageEntriesQuery,
        SearchSeriesQuery,
        GetForumThreadPostsQuery,
        TorrentSearch,
        ForumSearchQuery,
        TitleGroupCommentSearchQuery,
        DeleteForumCategoryQuery,
        DeleteForumSubCategoryQuery,
        DeleteForumThreadQuery,
        DeleteForumPostQuery,
        ForumSubCategoryAllowedPoster,
        GetForumSubCategoryAllowedPostersQuery,
        SearchTitleGroupTagsLiteQuery,
        SearchTitleGroupTagsQuery,
        SearchDonationsQuery,
        UserCreatedDonation,
        EditedDonation,
        DeletedDonation,
        DonationOrderBy,
        SearchUsersQuery,
        UserSearchOrderBy,
        SearchSentInvitationsQuery,
        InvitationSearchOrderByColumn,
        DeleteTorrentReportQuery,
        TorrentActivity,
        TorrentActivityAndTitleGroup,
        TorrentActivityOrderByColumn,
        TorrentActivitiesOverview,
        SeedersPerTorrent,
        ShopPurchase,
        ShopItem,
        ShopPricing,
        PromotionPricing,
        BuyUploadRequest,
        BuyFreeleechTokensRequest,
        UploadDiscountTier,
        FreeleechTokenDiscountTier,
        UploadPriceCalculation,
        FreeleechTokensPriceCalculation,
        TorrentStatsResponse,
        TorrentStatsDataPoint,
        StatsInterval,
        TorrentStatsGroupBy,
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

struct SideEffectsResponseModifier;

impl Modify for SideEffectsResponseModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components
            .schemas
            .insert("SideEffect".to_string(), SideEffect::schema());

        for path_item in openapi.paths.paths.values_mut() {
            let operations: Vec<&mut utoipa::openapi::path::Operation> = [
                &mut path_item.get,
                &mut path_item.put,
                &mut path_item.post,
                &mut path_item.delete,
                &mut path_item.options,
                &mut path_item.head,
                &mut path_item.patch,
                &mut path_item.trace,
            ]
            .into_iter()
            .filter_map(|op| op.as_mut())
            .collect();

            for operation in operations {
                for (status_code, response_ref) in operation.responses.responses.iter_mut() {
                    let is_success = status_code
                        .parse::<u16>()
                        .map(|code| (200..300).contains(&code))
                        .unwrap_or(false);
                    if !is_success {
                        continue;
                    }

                    let RefOr::T(response) = response_ref else {
                        continue;
                    };

                    let Some(content) = response.content.get("application/json") else {
                        continue;
                    };

                    let Some(original_schema) = content.schema.clone() else {
                        continue;
                    };

                    let wrapped_schema = Schema::Object(
                        Object::builder()
                            .property(
                                "side_effects",
                                Schema::Array(
                                    Array::builder()
                                        .items(RefOr::Ref(Ref::from_schema_name("SideEffect")))
                                        .build(),
                                ),
                            )
                            .required("side_effects")
                            .property("data", original_schema)
                            .required("data")
                            .build(),
                    );

                    *response_ref = RefOr::T(
                        ResponseBuilder::new()
                            .description(response.description.clone())
                            .content(
                                "application/json",
                                ContentBuilder::new().schema(Some(wrapped_schema)).build(),
                            )
                            .build(),
                    );
                }
            }
        }
    }
}
