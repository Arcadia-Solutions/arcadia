use arcadia_periodic_tasks::config::PeriodicTasksConfig;
use arcadia_shared::config::{default_log_level, DatabaseConfig, TelemetryConfig};
use arcadia_storage::models::arcadia_settings::HttpMethod;
use reqwest::Url;
use serde::Deserialize;
use std::net::IpAddr;
use std::num::NonZeroU32;

use crate::handlers::scrapers::ExternalSource;

/// Sections of the `config.yml` file used by the API. The other ones are ignored.
#[derive(Clone, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub tracker: TrackerConfig,
    pub periodic_tasks: PeriodicTasksConfig,
    #[serde(default)]
    pub smtp: SmtpConfig,
    #[serde(default)]
    pub image_host: ImageHostConfig,
    #[serde(default)]
    pub ergo: ErgoConfig,
    #[serde(default)]
    pub telemetry: TelemetryConfig,
    /// External sources provided by plugins, declared by the instance administrator.
    #[serde(default)]
    pub scrapers: Vec<ExternalSourcePlugin>,
    /// Optional: per subject rate limiting. Absent = rate limiting is disabled.
    #[serde(default)]
    pub rate_limits: Option<RateLimitsConfig>,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_api_port() -> u16 {
    8080
}

#[derive(Clone, Deserialize)]
pub struct ApiConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_api_port")]
    pub port: u16,
    pub jwt_secret: String,
    pub frontend_url: Url,
    /// Overridden by the `RUST_LOG` environment variable when it is set.
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default)]
    pub tmdb_api_key: Option<String>,
    #[serde(default)]
    pub comic_vine_api_key: Option<String>,
    /// When set, outgoing requests to external services are routed through this proxy.
    /// Internal requests (tracker, irc, image host, etc.) are not affected.
    #[serde(default)]
    pub http_proxy: Option<String>,
    /// Optional: header set by the reverse proxy holding the real ip address of the client.
    /// The last address of the comma separated list is selected. Leave it unset to select the
    /// connecting ip address when there is no reverse proxy. A reverse proxy that overwrites
    /// the header is required, otherwise a client can forge it and dodge the rate limits.
    #[serde(default)]
    pub reverse_proxy_client_ip_header_name: Option<String>,
}

fn default_redis_port() -> u16 {
    6379
}

#[derive(Clone, Deserialize)]
pub struct RedisConfig {
    #[serde(default = "default_host")]
    pub host: String,
    pub password: String,
    #[serde(default = "default_redis_port")]
    pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct TrackerConfig {
    pub name: String,
    pub url: Url,
    pub url_internal: Url,
    pub api_key: String,
    #[serde(default)]
    pub torrent_source_tag: Option<String>,
    /// Optional: the only address the tracker endpoints (`/api/tracker`) accept requests from.
    /// Unset = any address holding the api key reaches them.
    #[serde(default)]
    pub allowed_ip: Option<IpAddr>,
}

#[derive(Clone, Default, Deserialize)]
pub struct SmtpConfig {
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub from_email: Option<String>,
    #[serde(default)]
    pub from_name: Option<String>,
}

impl SmtpConfig {
    /// Emails are only sent when every key of the section is set.
    pub fn is_enabled(&self) -> bool {
        self.host.is_some()
            && self.port.is_some()
            && self.username.is_some()
            && self.password.is_some()
            && self.from_email.is_some()
            && self.from_name.is_some()
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct ErgoConfig {
    #[serde(default)]
    pub api_url: Option<String>,
    #[serde(default)]
    pub api_bearer_token: Option<String>,
    /// Token that Ergo sends when calling the auth callback endpoint.
    #[serde(default)]
    pub auth_callback_token: Option<String>,
}

impl ErgoConfig {
    pub fn is_enabled(&self) -> bool {
        self.api_url.is_some() && self.api_bearer_token.is_some()
    }
}

#[derive(Clone, Default, Deserialize)]
pub struct ImageHostConfig {
    #[serde(default)]
    pub chevereto_api_url: Option<String>,
    #[serde(default)]
    pub chevereto_api_key: Option<String>,
    /// Automatically rehost images coming from external database scrapers.
    #[serde(default)]
    pub rehost_external_images: bool,
}

fn default_timeout_seconds() -> u64 {
    30
}

/// An external source (scraper) provided by a plugin running as a separate service.
/// Declared by the instance administrator in the `scrapers` section of the configuration file.
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalSourcePlugin {
    /// What the interface needs to display the source: its `id` (used as the last segment of
    /// `/api/external-sources/{source_id}`), its `placeholder` and its `content_types`.
    #[serde(flatten)]
    pub source: ExternalSource,
    /// Endpoint of the plugin, called with the `url` query parameter.
    pub url: String,
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
}

/// Quota applied to every request that no rule matches.
#[derive(Clone, Copy, Deserialize)]
pub struct RateLimitQuotaConfig {
    pub requests: NonZeroU32,
    pub per_seconds: NonZeroU32,
}

/// A single rule of the ordered list. `method` applies to every method when it is unset.
#[derive(Clone, Deserialize)]
pub struct RateLimitRuleConfig {
    pub path_prefix: String,
    #[serde(default)]
    pub method: Option<HttpMethod>,
    pub requests: NonZeroU32,
    pub per_seconds: NonZeroU32,
}

/// The rules are matched in the order they are written in the configuration file, and the
/// first match wins.
#[derive(Clone, Deserialize)]
pub struct RateLimitsConfig {
    pub default: RateLimitQuotaConfig,
    #[serde(default)]
    pub rules: Vec<RateLimitRuleConfig>,
}
