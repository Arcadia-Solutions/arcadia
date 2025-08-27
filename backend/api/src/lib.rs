use arcadia_storage::connection_pool::ConnectionPool;
use std::{ops::Deref, str::FromStr, sync::Arc};

use crate::env::Env;

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

pub struct Arcadia {
    pub pool: Arc<ConnectionPool>,
    env: Env,
}

impl Deref for Arcadia {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Arcadia {
    pub fn new(pool: Arc<ConnectionPool>, env: Env) -> Self {
        Self { pool, env }
    }
    #[inline]
    pub fn is_open_signups(&self) -> bool {
        Into::<OpenSignups>::into(self.env.open_signups) == OpenSignups::Enabled
    }
}
