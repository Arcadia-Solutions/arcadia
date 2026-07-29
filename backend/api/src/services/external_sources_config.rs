use serde::Deserialize;

use crate::handlers::scrapers::ExternalSource;

/// Read from the working directory of the backend, like the `.env` file.
const PLUGINS_CONFIG_FILE: &str = "plugins.yml";

fn default_timeout_seconds() -> u64 {
    30
}

/// An external source (scraper) provided by a plugin running as a separate service.
/// Declared by the instance administrator in the plugins configuration file.
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalSourcePlugin {
    /// What the interface needs to display the source: its `id` (used as the last segment of
    /// `/api/external-sources/{source_id}`), its `label`, its `placeholder` and its `content_types`.
    #[serde(flatten)]
    pub source: ExternalSource,
    /// Endpoint of the plugin, called with the `url` query parameter.
    pub url: String,
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct PluginsConfig {
    #[serde(default)]
    pub scrapers: Vec<ExternalSourcePlugin>,
}

/// Loads the plugins configuration file. When it does not exist, no plugin is registered.
pub fn load_external_source_plugins() -> Vec<ExternalSourcePlugin> {
    let Ok(contents) = std::fs::read_to_string(PLUGINS_CONFIG_FILE) else {
        return Vec::new();
    };

    let config: PluginsConfig = serde_norway::from_str(&contents).unwrap_or_else(|error| {
        panic!("cannot parse the plugins configuration file '{PLUGINS_CONFIG_FILE}': {error}")
    });

    for plugin in &config.scrapers {
        println!("External source plugin registered: {}", plugin.source.id);
    }

    config.scrapers
}
