use serde::{de::DeserializeOwned, Deserialize};
use std::path::{Path, PathBuf};

/// Name of the single configuration file of the project, located at the root of the repository.
const CONFIGURATION_FILE_NAME: &str = "config.yml";

/// Environment variable holding an explicit path to the configuration file. When it is not set,
/// the file is looked up in the current directory and its parents.
const CONFIGURATION_PATH_VARIABLE: &str = "ARCADIA_CONFIG";

/// Locates the configuration file: the path given by `ARCADIA_CONFIG` when it is set, otherwise
/// the first `config.yml` found walking up from the current directory. Walking up is what lets
/// the services run both from the repository root and from their own crate directory.
pub fn configuration_file_path() -> PathBuf {
    if let Ok(path) = std::env::var(CONFIGURATION_PATH_VARIABLE) {
        return PathBuf::from(path);
    }

    let current_directory =
        std::env::current_dir().expect("cannot read the current working directory");

    let mut directory: Option<&Path> = Some(current_directory.as_path());
    while let Some(candidate_directory) = directory {
        let candidate = candidate_directory.join(CONFIGURATION_FILE_NAME);
        if candidate.is_file() {
            return candidate;
        }
        directory = candidate_directory.parent();
    }

    panic!(
        "no '{CONFIGURATION_FILE_NAME}' found in '{}' or any of its parent directories. \
         Copy 'config.example.yml' to '{CONFIGURATION_FILE_NAME}', or set the \
         '{CONFIGURATION_PATH_VARIABLE}' environment variable to its path.",
        current_directory.display()
    );
}

/// Reads and parses the configuration file. Every service deserializes only the sections it
/// needs, the other ones are ignored.
pub fn load<T: DeserializeOwned>() -> T {
    let path = configuration_file_path();

    let contents = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "cannot read the configuration file '{}': {error}",
            path.display()
        )
    });

    serde_norway::from_str(&contents).unwrap_or_else(|error| {
        panic!(
            "cannot parse the configuration file '{}': {error}",
            path.display()
        )
    })
}

#[derive(Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    /// Connection string given to sqlx.
    pub fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

/// Keeps the password out of any debug output.
impl std::fmt::Debug for DatabaseConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DatabaseConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("user", &self.user)
            .field("password", &"[redacted]")
            .field("name", &self.name)
            .finish()
    }
}

/// Default of the `log_level` key of every service.
pub fn default_log_level() -> String {
    "info,sqlx=info".to_string()
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TelemetryConfig {
    /// OTLP gRPC endpoint. When it is not set, telemetry is only written to stdout.
    /// Shared by every service, the log level is configured per service.
    #[serde(default)]
    pub otlp_endpoint: Option<String>,
}
