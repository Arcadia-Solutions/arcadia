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
    #[envconfig(nested)]
    pub image_host: ImageHostConfig,
    #[envconfig(from = "OTEL_SERVICE_NAME")]
    pub otel_service_name: Option<String>,
    #[envconfig(from = "BONUS_POINTS_FORMULA")]
    pub bonus_points_formula: String,
    #[envconfig(from = "TASK_INTERVAL_SEEDTIME_AND_BONUS_POINTS_UPDATE_SECONDS")]
    pub seedtime_and_bonus_points_update_seconds: u64,
    #[envconfig(nested)]
    pub ergo: ErgoConfig,
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
    // set in the smtp initializer. if it fails, leave it false, otherwise true
    #[envconfig(default = "false")]
    pub emails_enabled: bool,
}

#[derive(Envconfig, Clone)]
pub struct ErgoConfig {
    #[envconfig(from = "ERGO_API_URL")]
    pub api_url: Option<String>,
    #[envconfig(from = "ERGO_API_BEARER_TOKEN")]
    pub api_bearer_token: Option<String>,
    #[envconfig(from = "ERGO_AUTH_CALLBACK_TOKEN")]
    pub auth_callback_token: Option<String>,
}

impl ErgoConfig {
    pub fn is_enabled(&self) -> bool {
        self.api_url.is_some() && self.api_bearer_token.is_some()
    }
}

#[derive(Envconfig, Clone)]
pub struct ImageHostConfig {
    #[envconfig(from = "CHEVERETO_API_URL")]
    pub chevereto_api_url: Option<String>,
    #[envconfig(from = "CHEVERETO_API_KEY")]
    pub chevereto_api_key: Option<String>,
    #[envconfig(from = "REHOST_EXTERNAL_IMAGES", default = "false")]
    pub rehost_external_images: bool,
}
