pub mod error;

use self::error::{RedisError, Result};
use deadpool_redis::{Config, Connection, Pool, Runtime};
use redis::{cmd, ToRedisArgs};

pub struct RedisPool(Pool);

impl RedisPool {
    pub fn new(redis_host: &str, password: &str, port: u16) -> Self {
        let conn_string = format!("redis://:{}@{}:{}", password, redis_host, port);
        let config = Config::from_url(conn_string);
        let pool = config.create_pool(Some(Runtime::Tokio1)).unwrap();

        Self(pool)
    }

    pub async fn connection(&self) -> Result<Redis> {
        let conn = self.0.get().await.map_err(RedisError::ConnectionError)?;
        Ok(Redis::new(conn))
    }
}

pub struct Redis(Connection);

impl Redis {
    fn new(connection: Connection) -> Self {
        Redis(connection)
    }

    pub async fn set<S: ToRedisArgs>(&mut self, key: S, value: S) -> Result<()> {
        cmd("SET")
            .arg(&[key, value])
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn set_ex<K, V>(&mut self, key: K, value: V, secs: usize) -> Result<()>
    where
        K: ToRedisArgs,
        V: ToRedisArgs,
    {
        cmd("SETEX")
            .arg(key)
            .arg(secs)
            .arg(value)
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn get<K: ToRedisArgs>(&mut self, key: K) -> Result<Option<String>> {
        cmd("GET")
            .arg(&[key])
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn delete<K: ToRedisArgs>(&mut self, key: K) -> Result<()> {
        cmd("DEL")
            .arg(key)
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn keys<K: ToRedisArgs>(&mut self, key_pattern: K) -> Result<()> {
        cmd("keys")
            .arg(key_pattern)
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }
}
