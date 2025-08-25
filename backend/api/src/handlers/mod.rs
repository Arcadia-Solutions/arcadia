pub mod announce_handler;
pub mod artist_handler;
pub mod auth_handler;
pub mod conversation_handler;
pub mod edition_group_handler;
pub mod forum_handler;
pub mod gift_handler;
pub mod home_handler;
pub mod invitation_handler;
pub mod master_group_handler;
pub mod peers_handler;
pub mod scrapers;
pub mod series_handler;
pub mod subscriptions_handler;
pub mod title_group_comment_handler;
pub mod title_group_handler;
pub mod torrent_handler;
pub mod torrent_report_handler;
pub mod torrent_request_handler;
pub mod torrent_request_vote_handler;
pub mod user_application_handler;
pub mod user_handler;
pub mod wiki_handler;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use actix_web::HttpMessage as _;
use arcadia_storage::models::user;

// Populated by the authentication middleware.
#[derive(Debug, Copy, Clone)]
pub struct UserId(pub i64);

impl actix_web::FromRequest for UserId {
    type Error = std::convert::Infallible;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let user_id = *req
            .extensions()
            .get::<UserId>()
            .expect("user id matcher used on unauthenticated endpoint");

        std::future::ready(Ok(user_id))
    }
}

pub struct User(user::User);

impl Deref for User {
    type Target = user::User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for User {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl actix_web::FromRequest for User {
    type Error = actix_web::Error;
    type Future = futures::future::LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let arc = req
            .app_data::<actix_web::web::Data<crate::Arcadia>>()
            .expect("arcadia should be setup");

        let user_id = req
            .extensions()
            .get::<UserId>()
            .expect("user id should be setup")
            .0;

        let pool = Arc::clone(&arc.pool);

        Box::pin(async move {
            pool.find_user_with_id(user_id)
                .await
                .map(User)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
        })
    }
}
