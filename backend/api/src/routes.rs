use actix_web::web::{self, scope};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::handlers::affiliated_artists::config as AffiliatedArtistsConfig;
use crate::handlers::announces::config as AnnouncesConfig;
use crate::handlers::artists::config as ArtistsConfig;
use crate::handlers::auth::config as AuthConfig;
use crate::handlers::conversations::config as ConversationsConfig;
use crate::handlers::edition_groups::config as EditionGroupsConfig;
use crate::handlers::external_db::config as ExternalDbConfig;
use crate::handlers::forum::config as ForumConfig;
use crate::handlers::gifts::config as GiftsConfig;
use crate::handlers::home::config as HomeConfig;
use crate::handlers::invitations::config as InvitationsConfig;
use crate::handlers::master_groups::config as MasterGroupsConfig;
use crate::handlers::search::config as SearchConfig;
use crate::handlers::series::config as SeriesConfig;
use crate::handlers::subscriptions::config as SubscriptionsConfig;
use crate::handlers::title_groups::config as TitleGroupsConfig;
use crate::handlers::torrent_requests::config as TorrentRequestsConfig;
use crate::handlers::torrents::config as TorrentsConfig;
use crate::handlers::user_applications::config as UserApplicationsConfig;
use crate::handlers::users::config as UsersConfig;
use crate::handlers::wiki::config as WikiConfig;
use crate::middlewares::jwt_middleware::authenticate_user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/announce").configure(AnnouncesConfig));

    cfg.service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .service(scope("/auth").configure(AuthConfig))
            .service(scope("/users").configure(UsersConfig))
            .service(scope("/user-applications").configure(UserApplicationsConfig))
            .service(scope("/title-groups").configure(TitleGroupsConfig))
            .service(scope("/edition-groups").configure(EditionGroupsConfig))
            .service(scope("/search").configure(SearchConfig))
            .service(scope("/torrents").configure(TorrentsConfig))
            .service(scope("/torrent-requests").configure(TorrentRequestsConfig))
            .service(scope("/artists").configure(ArtistsConfig))
            .service(scope("/affiliated-artists").configure(AffiliatedArtistsConfig))
            .service(scope("/conversations").configure(ConversationsConfig))
            .service(scope("/subscriptions").configure(SubscriptionsConfig))
            .service(scope("/series").configure(SeriesConfig))
            .service(scope("/external-sources").configure(ExternalDbConfig))
            .service(scope("/forum").configure(ForumConfig))
            .service(scope("/wiki").configure(WikiConfig))
            .service(scope("/invitations").configure(InvitationsConfig))
            .service(scope("/home").configure(HomeConfig))
            .service(scope("/master-groups").configure(MasterGroupsConfig))
            .service(scope("/gifts").configure(GiftsConfig)),
    );
}
