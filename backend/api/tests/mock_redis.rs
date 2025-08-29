use std::collections::HashMap;

use arcadia_storage::redis::{error::Result, RedisInterface, RedisPoolInterface};
use redis::ToRedisArgs;
pub struct RedisPool;

impl RedisPoolInterface for RedisPool {
    async fn connection(&self) -> Result<impl RedisInterface> {
        Ok(Redis::new())
    }
}

struct Redis {
    inner: HashMap<Vec<u8>, Vec<u8>>,
}

impl Redis {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl RedisInterface for Redis {
    async fn set<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: ToRedisArgs + Send,
        V: ToRedisArgs + Send,
    {
        let key = key.to_redis_args()[0].clone();
        let value = value.to_redis_args()[0].clone();

        self.inner.insert(key, value);
        Ok(())
    }

    async fn set_ex<K, V>(&mut self, key: K, value: V, _: usize) -> Result<()>
    where
        K: ToRedisArgs + Send,
        V: ToRedisArgs + Send,
    {
        self.set(key, value).await
    }

    async fn get<K: ToRedisArgs + Send>(&mut self, key: K) -> Result<Option<String>> {
        let key = key.to_redis_args()[0].clone();
        Ok(self
            .inner
            .get(&key)
            .map(|v| str::from_utf8(v).unwrap().to_string()))
    }

    async fn delete<K: ToRedisArgs + Send>(&mut self, key: K) -> Result<()> {
        let key = key.to_redis_args()[0].clone();
        self.inner.remove(&key);
        Ok(())
    }
}
