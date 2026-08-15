use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::user::{Claims, LoginResponse},
    redis::{RedisInterface, RedisPool, RedisPoolInterface},
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::sync::{Arc, LazyLock};

pub static REFRESH_TOKEN_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::days(90));
pub static AUTH_TOKEN_SHORT_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::hours(1));
pub static AUTH_TOKEN_LONG_DURATION: LazyLock<Duration> = LazyLock::new(|| Duration::days(1));

/// Issues an authentication token for the user, along with a refresh token when the session
/// is meant to outlive the short lived authentication token.
///
/// `issued_at` is the moment the tokens are issued at. A token issued after an invalidation
/// stays valid (see [`Auth::is_invalidated`]), so a caller invalidating the user's sessions and
/// keeping them logged in must issue the new tokens strictly after the invalidation.
pub fn generate_login_tokens(
    user_id: i32,
    jwt_secret: &str,
    remember_me: bool,
    issued_at: DateTime<Utc>,
) -> Result<LoginResponse> {
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let authentication_token_duration = if remember_me {
        *AUTH_TOKEN_LONG_DURATION
    } else {
        *AUTH_TOKEN_SHORT_DURATION
    };

    let encode_token = |expiration_date: DateTime<Utc>| {
        encode(
            &Header::default(),
            &Claims {
                sub: user_id,
                exp: expiration_date.timestamp(),
                iat: issued_at.timestamp(),
            },
            &encoding_key,
        )
        .map_err(Error::JwtError)
    };

    let token = encode_token(issued_at + authentication_token_duration)?;
    let refresh_token = if remember_me {
        encode_token(issued_at + *REFRESH_TOKEN_DURATION)?
    } else {
        String::new()
    };

    Ok(LoginResponse {
        token,
        refresh_token,
    })
}

#[derive(Serialize, Deserialize)]
pub struct InvalidationEntry {
    user_id: i32,
    token_invalidation_ts: i64,
}

impl InvalidationEntry {
    pub fn new(user_id: i32) -> Self {
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

    pub async fn invalidate(&self, user_id: i32) -> Result<()> {
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

    pub async fn is_invalidated(&self, user_id: i32, iat: i64) -> Result<bool> {
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
