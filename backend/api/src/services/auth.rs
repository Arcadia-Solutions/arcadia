use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPool;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
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

pub struct TokenValidation {
    redis_pool: Arc<RedisPool>,
}

impl TokenValidation {
    pub fn new(redis_pool: Arc<RedisPool>) -> Self {
        Self { redis_pool }
    }

    pub async fn invalidate(&self, user_id: i64) -> Result<()> {
        let entry = InvalidationEntry::new(user_id);
        let mut redis = self.redis_pool.connection().await?;
        redis
            .set_ex(
                user_id,
                to_string(&entry)?,
                (*REFRESH_TOKEN_DURATION).as_seconds_f64() as usize,
            )
            .await?;
        Ok(())
    }
}
