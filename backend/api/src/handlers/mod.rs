pub mod affiliated_artists;
pub mod announces;
pub mod artists;
pub mod auth;
pub mod conversations;
pub mod edition_groups;
pub mod external_db;
pub mod forum;
pub mod gifts;
pub mod home;
pub mod invitations;
pub mod master_groups;
pub mod search;
pub mod series;
pub mod subscriptions;
pub mod title_groups;
pub mod torrent_requests;
pub mod torrents;
pub mod user_applications;
pub mod users;
pub mod wiki;

pub mod peers_handler;
pub mod scrapers;

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use actix_web::HttpMessage as _;
use arcadia_storage::models::user;

// Populated by the authentication middleware.
#[derive(Debug, Copy, Clone)]
pub struct UserId(pub i64);

impl Deref for UserId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
