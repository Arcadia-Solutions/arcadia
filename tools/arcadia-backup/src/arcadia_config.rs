//! The sections of the Arcadia `config.yml` the backups need, read from the Arcadia host.

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArcadiaFile {
    pub database: Database,
    #[serde(default)]
    pub redis: Redis,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    #[serde(default = "default_redis_host")]
    pub host: String,
    #[serde(default = "default_redis_port")]
    pub port: u16,
    #[serde(default)]
    pub password: String,
}

impl Default for Redis {
    fn default() -> Self {
        Redis {
            host: default_redis_host(),
            port: default_redis_port(),
            password: String::new(),
        }
    }
}

fn default_redis_host() -> String {
    "127.0.0.1".to_string()
}

fn default_redis_port() -> u16 {
    6379
}

pub fn parse(contents: &str) -> Result<ArcadiaFile> {
    yaml_serde::from_str(contents).context("cannot parse the config.yml of the Arcadia host")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_example_configuration() {
        let config = parse(include_str!("../../../config.example.yml")).unwrap();
        assert_eq!(config.database.user, "arcadia");
        assert_eq!(config.database.port, 5432);
        assert_eq!(config.redis.port, 6379);
    }

    #[test]
    fn redis_host_and_port_default() {
        let config = parse(include_str!("../../../config.ci.yml")).unwrap();
        assert_eq!(config.database.host, "arcadiadb");
        assert_eq!(config.redis.host, "127.0.0.1");
        assert_eq!(config.redis.password, "");
    }

    #[test]
    fn database_section_is_required() {
        assert!(parse("redis:\n  password: x\n").is_err());
    }
}
