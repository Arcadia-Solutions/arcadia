use bincode::config;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode, PartialEq)]
pub struct ArcadiaSettingsForTracker {
    pub global_upload_factor: i16,
    pub global_download_factor: i16,
}

impl ArcadiaSettingsForTracker {
    pub async fn from_backend() -> Self {
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/env", base_url);

        let client = Client::new();
        let api_key = std::env::var("API_KEY").expect("env var API_KEY not set");
        let resp = client
            .get(url)
            .header("api_key", api_key)
            .send()
            .await
            .expect("failed to fetch env");
        let bytes = resp
            .bytes()
            .await
            .expect("failed to read env response body");

        let config = config::standard();
        let (env, _): (ArcadiaSettingsForTracker, usize) =
            bincode::decode_from_slice(&bytes[..], config).unwrap();
        env
    }
}
