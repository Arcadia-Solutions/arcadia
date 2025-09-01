use arcadia_storage::{connection_pool::ConnectionPool, redis::RedisPoolInterface};
use std::{ops::Deref, str::FromStr, sync::Arc};

use crate::{env::Env, services::auth::Auth};

pub mod api_doc;
pub mod env;
pub mod handlers;
pub mod middlewares;
pub mod routes;
pub mod services;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpenSignups {
    Disabled,
    Enabled,
}

impl FromStr for OpenSignups {
    type Err = env::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self::Enabled),
            "false" => Ok(Self::Disabled),
            _ => Err(env::Error::EnvVariableParseError(
                "ARCADIA_OPEN_SIGNUPS".to_string(),
            )),
        }
    }
}

pub struct Arcadia<R: RedisPoolInterface> {
    pub pool: Arc<ConnectionPool>,
    pub redis_pool: Arc<R>,
    pub auth: Auth<R>,
    env: Env,
}

impl<R: RedisPoolInterface> Deref for Arcadia<R> {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl<R: RedisPoolInterface> Arcadia<R> {
    pub fn new(pool: Arc<ConnectionPool>, redis_pool: Arc<R>, env: Env) -> Self {
        Self {
            pool,
            redis_pool: Arc::clone(&redis_pool),
            auth: Auth::new(Arc::clone(&redis_pool)),
            env,
        }
    }
    #[inline]
    pub fn is_open_signups(&self) -> bool {
        Into::<OpenSignups>::into(self.env.open_signups) == OpenSignups::Enabled
    }
}
