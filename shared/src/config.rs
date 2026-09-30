use serde::{de::DeserializeOwned, Deserialize};
use std::path::{Path, PathBuf};

/// Name of the single configuration file of the project, located at the root of the repository.
const CONFIGURATION_FILE_NAME: &str = "config.yml";

/// Environment variable holding an explicit path to the configuration file. When it is not set,
/// the file is looked up in the current directory and its parents.
const CONFIGURATION_PATH_VARIABLE: &str = "ARCADIA_CONFIG";

/// Locates the configuration file: the path given by `ARCADIA_CONFIG` when it is set, otherwise
/// the first `config.yml` found walking up from the current directory.
pub fn find_configuration_file() -> Option<PathBuf> {
    if let Ok(path) = std::env::var(CONFIGURATION_PATH_VARIABLE) {
        return Some(PathBuf::from(path));
    }

    let current_directory = std::env::current_dir().ok()?;
    let mut directory: Option<&Path> = Some(current_directory.as_path());
    while let Some(candidate_directory) = directory {
        let candidate = candidate_directory.join(CONFIGURATION_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
        directory = candidate_directory.parent();
    }

    None
}


fn set_nested_value(root: &mut serde_norway::Value, segments: &[String], val: serde_norway::Value) {
    if segments.is_empty() {
        return;
    }
    if !root.is_mapping() {
        *root = serde_norway::Value::Mapping(serde_norway::Mapping::new());
    }
    let mut current = root;
    for (i, segment) in segments.iter().enumerate() {
        let is_last = i == segments.len() - 1;
        let mapping = current.as_mapping_mut().unwrap();
        let key = serde_norway::Value::String(segment.clone());
        if is_last {
            mapping.insert(key, val);
            return;
        } else {
            if !mapping.contains_key(&key) || !mapping[&key].is_mapping() {
                mapping.insert(
                    key.clone(),
                    serde_norway::Value::Mapping(serde_norway::Mapping::new()),
                );
            }
            current = mapping.get_mut(&key).unwrap();
        }
    }
}

/// Overlays environment variables onto a `serde_norway::Value` configuration tree.
///
/// Precedence:
/// 1. `ARCADIA_<SECTION>__<KEY>`
/// 2. Underlying YAML values in `root`
pub fn merge_env_vars(
    root: &mut serde_norway::Value,
    env_vars: impl IntoIterator<Item = (String, String)>,
) {
    for (key, val) in env_vars {
        if key == CONFIGURATION_PATH_VARIABLE {
            continue;
        }

        let Some(stripped) = key.strip_prefix("ARCADIA_") else {
            continue;
        };

        let segments: Vec<String> = stripped
            .split("__")
            .map(|s| s.to_lowercase())
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            continue;
        }

        let parsed_val = serde_norway::from_str::<serde_norway::Value>(&val)
            .unwrap_or(serde_norway::Value::String(val));

        set_nested_value(root, &segments, parsed_val);
    }
}

/// Reads and parses the configuration file, overlaid with environment variables.
///
/// Every service deserializes only the sections it needs; other sections are ignored.
pub fn load<T: DeserializeOwned>() -> T {
    let mut root: serde_norway::Value = match find_configuration_file() {
        Some(path) => {
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
        None => serde_norway::Value::Mapping(serde_norway::Mapping::new()),
    };

    merge_env_vars(&mut root, std::env::vars());

    serde_norway::from_value(root).unwrap_or_else(|error| {
        panic!(
            "cannot load configuration: {error}. \
             Ensure '{CONFIGURATION_FILE_NAME}' exists or provide required settings via \
             'ARCADIA_<SECTION>__<KEY>' environment variables."
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct TestConfig {
        database: DatabaseConfig,
        #[serde(default)]
        api: Option<TestApiConfig>,
    }

    #[derive(Debug, Deserialize)]
    struct TestApiConfig {
        port: u16,
        host: String,
        debug: bool,
    }

    #[test]
    fn test_merge_env_vars_overrides_yaml() {
        let yaml = r#"
database:
  host: localhost
  port: 5432
  user: arcadia
  password: old_password
  name: arcadia
api:
  port: 8080
  host: 127.0.0.1
  debug: false
"#;
        let mut root: serde_norway::Value = serde_norway::from_str(yaml).unwrap();
        let envs = vec![
            ("ARCADIA_DATABASE__PASSWORD".into(), "new_secret".into()),
            ("ARCADIA_API__PORT".into(), "9090".into()),
            ("ARCADIA_API__DEBUG".into(), "true".into()),
        ];
        merge_env_vars(&mut root, envs);
        let parsed: TestConfig = serde_norway::from_value(root).unwrap();

        assert_eq!(parsed.database.password, "new_secret");
        assert_eq!(parsed.database.port, 5432);
        assert_eq!(parsed.api.as_ref().unwrap().port, 9090);
        assert_eq!(parsed.api.as_ref().unwrap().host, "127.0.0.1");
        assert!(parsed.api.as_ref().unwrap().debug);
    }
}
