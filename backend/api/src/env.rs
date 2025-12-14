use envconfig::Envconfig;
use reqwest::Url;

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(nested)]
    pub actix: ActixConfig,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
    #[envconfig(from = "ARCADIA_USER_CLASS_NAME_ON_SIGNUP")]
    pub user_class_name_on_signup: String,
    #[envconfig(from = "ARCADIA_FRONTEND_URL")]
    pub frontend_url: Url,
    #[envconfig(nested)]
    pub tracker: TrackerConfig,
    #[envconfig(nested)]
    pub smtp: SmtpConfig,
    #[envconfig(nested)]
    pub redis: RedisConfig,
    #[envconfig(from = "TMDB_API_KEY")]
    pub tmdb_api_key: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("env variable parse error '{0}'")]
    EnvVariableParseError(String),
}

#[derive(Envconfig, Clone)]
pub struct ActixConfig {
    #[envconfig(from = "ACTIX_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "ACTIX_PORT", default = "8080")]
    pub port: u16,
}

#[derive(Envconfig, Clone)]
pub struct RedisConfig {
    #[envconfig(from = "REDIS_HOST", default = "127.0.0.1")]
    pub host: String,
    #[envconfig(from = "REDIS_PASSWORD")]
    pub password: String,
    #[envconfig(from = "REDIS_PORT", default = "6379")]
    pub port: u16,
}

#[derive(Envconfig, Clone)]
pub struct TrackerConfig {
    #[envconfig(from = "ARCADIA_TRACKER_NAME")]
    pub name: String,
    #[envconfig(from = "ARCADIA_TRACKER_URL")]
    pub url: Url,
    #[envconfig(from = "ARCADIA_TRACKER_URL_INTERNAL")]
    pub url_internal: Url,

    #[envconfig(from = "ARCADIA_TRACKER_API_KEY")]
    pub api_key: String,
}

#[derive(Envconfig, Clone)]
pub struct SmtpConfig {
    #[envconfig(from = "SMTP_HOST")]
    pub host: Option<String>,
    #[envconfig(from = "SMTP_PORT")]
    pub port: Option<u16>,
    #[envconfig(from = "SMTP_USERNAME")]
    pub username: Option<String>,
    #[envconfig(from = "SMTP_PASSWORD")]
    pub password: Option<String>,
    #[envconfig(from = "SMTP_FROM_EMAIL")]
    pub from_email: Option<String>,
    #[envconfig(from = "SMTP_FROM_NAME")]
    pub from_name: Option<String>,
}
