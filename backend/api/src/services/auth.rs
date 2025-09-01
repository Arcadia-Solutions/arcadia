use arcadia_common::error::Result;
use arcadia_storage::redis::{RedisInterface, RedisPool, RedisPoolInterface};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::sync::{Arc, LazyLock};

pub static REFRESH_TOKEN_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::days(90));
pub static AUTH_TOKEN_SHORT_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::hours(1));
pub static AUTH_TOKEN_LONG_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::days(1));

#[derive(Serialize, Deserialize)]
pub struct InvalidationEntry {
    user_id: i64,
    token_invalidation_ts: i64,
}

impl InvalidationEntry {
    pub fn new(user_id: i64) -> Self {
        let now = Utc::now();

        Self {
            user_id,
            token_invalidation_ts: now.timestamp(),
        }
    }
}

pub struct Auth<R: RedisPoolInterface = RedisPool> {
    redis_pool: Arc<R>,
}

impl<R: RedisPoolInterface> Auth<R> {
    pub fn new(redis_pool: Arc<R>) -> Self {
        Self { redis_pool }
    }

    pub async fn invalidate(&self, user_id: i64) -> Result<()> {
        let entry = InvalidationEntry::new(user_id);
        let mut redis = self.redis_pool.connection().await?;

        // add entry to the redis with a TTL of the refresh token so we know
        // for sure that it will be present for as long as the refresh token is present
        redis
            .set_ex(
                user_id,
                to_string(&entry)?,
                (*REFRESH_TOKEN_DURATION).as_seconds_f64() as usize,
            )
            .await?;
        Ok(())
    }

    pub async fn is_invalidated(&self, user_id: i64, iat: i64) -> Result<bool> {
        let mut redis = self.redis_pool.connection().await?;
        let Some(entry) = redis.get(user_id).await? else {
            return Ok(false);
        };

        let entry: InvalidationEntry = from_str(&entry)?;

        // a token that is issued after the invalidation date is valid as it's a fresh one
        // whereas old tokens should be treated as invalid
        if iat > entry.token_invalidation_ts {
            return Ok(false);
        }

        Ok(true)
    }
}
