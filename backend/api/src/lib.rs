use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::{arcadia_settings::ArcadiaSettings, notification::NotificationEvent},
    redis::RedisPoolInterface,
};
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use crate::{env::Env, services::auth::Auth};

pub mod api_doc;
pub mod env;
pub mod handlers;
pub mod middlewares;
pub mod routes;
pub mod services;

pub struct Arcadia<R: RedisPoolInterface> {
    pub pool: Arc<ConnectionPool>,
    pub redis_pool: Arc<R>,
    pub auth: Auth<R>,
    pub settings: Arc<Mutex<ArcadiaSettings>>,
    pub notification_sender: broadcast::Sender<NotificationEvent>,
    env: Env,
}

impl<R: RedisPoolInterface> Deref for Arcadia<R> {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl<R: RedisPoolInterface> Arcadia<R> {
    pub fn new(
        pool: Arc<ConnectionPool>,
        redis_pool: Arc<R>,
        env: Env,
        settings: ArcadiaSettings,
    ) -> Self {
        let (notification_sender, _) = broadcast::channel(256);

        Self {
            pool,
            redis_pool: Arc::clone(&redis_pool),
            auth: Auth::new(Arc::clone(&redis_pool)),
            settings: Arc::new(Mutex::new(settings)),
            notification_sender,
            env,
        }
    }
}
