use actix_web::web::{self, scope};
use actix_web_httpauth::middleware::HttpAuthentication;
use arcadia_storage::redis::RedisPoolInterface;

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

pub fn init<R: RedisPoolInterface + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.service(scope("/announce").configure(AnnouncesConfig::<R>));

    cfg.service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(authenticate_user::<R>))
            .service(scope("/auth").configure(AuthConfig::<R>))
            .service(scope("/users").configure(UsersConfig::<R>))
            .service(scope("/user-applications").configure(UserApplicationsConfig::<R>))
            .service(scope("/title-groups").configure(TitleGroupsConfig::<R>))
            .service(scope("/edition-groups").configure(EditionGroupsConfig::<R>))
            .service(scope("/search").configure(SearchConfig::<R>))
            .service(scope("/torrents").configure(TorrentsConfig::<R>))
            .service(scope("/torrent-requests").configure(TorrentRequestsConfig::<R>))
            .service(scope("/artists").configure(ArtistsConfig::<R>))
            .service(scope("/affiliated-artists").configure(AffiliatedArtistsConfig::<R>))
            .service(scope("/conversations").configure(ConversationsConfig::<R>))
            .service(scope("/subscriptions").configure(SubscriptionsConfig::<R>))
            .service(scope("/series").configure(SeriesConfig::<R>))
            .service(scope("/external-sources").configure(ExternalDbConfig::<R>))
            .service(scope("/forum").configure(ForumConfig::<R>))
            .service(scope("/wiki").configure(WikiConfig::<R>))
            .service(scope("/invitations").configure(InvitationsConfig::<R>))
            .service(scope("/home").configure(HomeConfig::<R>))
            .service(scope("/master-groups").configure(MasterGroupsConfig::<R>))
            .service(scope("/gifts").configure(GiftsConfig::<R>)),
    );
}
