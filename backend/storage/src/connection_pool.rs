use sqlx::{postgres::PgPoolOptions, PgPool};
use std::borrow::{Borrow, BorrowMut};
use url::Url;

#[derive(Clone)]
pub struct TrackerConfig {
    pub url_internal: Url,
    pub api_key: String,
}

pub struct ConnectionPool {
    pool: PgPool,
    pub tracker_config: TrackerConfig,
}

impl ConnectionPool {
    pub async fn try_new(db_uri: &str, tracker_config: TrackerConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_uri)
            .await
            .expect("Error building a connection pool");

        Ok(Self {
            pool,
            tracker_config,
        })
    }

    /// Creates a ConnectionPool from an existing PgPool, without tracker config.
    /// Used in tests where tracker notifications are disabled.
    pub fn with_pg_pool(pool: PgPool) -> Self {
        Self {
            pool,
            // Dummy config for tests - notifications are disabled via notify_tracker parameter
            tracker_config: TrackerConfig {
                url_internal: Url::parse("http://localhost").unwrap(),
                api_key: String::new(),
            },
        }
    }
}

impl Borrow<PgPool> for ConnectionPool {
    fn borrow(&self) -> &PgPool {
        &self.pool
    }
}

impl BorrowMut<PgPool> for ConnectionPool {
    fn borrow_mut(&mut self) -> &mut PgPool {
        &mut self.pool
    }
}
