pub mod error;

use self::error::{RedisError, Result};
use deadpool_redis::{Config, Connection, Pool, Runtime};
use redis::cmd;

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

    pub async fn set(&mut self, key: &str, value: &str) -> Result<()> {
        cmd("SET")
            .arg(&[key, value])
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn set_ex(&mut self, key: &str, value: &str, secs: usize) -> Result<()> {
        cmd("SETEX")
            .arg(key)
            .arg(secs)
            .arg(value)
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn get(&mut self, key: &str) -> Result<String> {
        cmd("GET")
            .arg(&[key])
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn delete(&mut self, key: &str) -> Result<()> {
        cmd("DEL")
            .arg(key)
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }

    pub async fn keys(&mut self, key_pattern: &str) -> Result<()> {
        cmd("keys")
            .arg(key_pattern)
            .query_async(&mut self.0)
            .await
            .map_err(RedisError::CmdError)
    }
}
