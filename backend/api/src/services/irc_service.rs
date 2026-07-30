use crate::Arcadia;
use arcadia_common::error::{Error, Result};
use arcadia_storage::redis::RedisPoolInterface;
use rand::RngExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct IrcService {
    api_url: String,
    api_bearer_token: String,
    client: Client,
}

#[derive(Serialize)]
struct SaregisterRequest {
    #[serde(rename = "accountName")]
    account_name: String,
    passphrase: String,
}

#[derive(Deserialize)]
struct ErgoApiResponse {
    success: bool,
    error: Option<String>,
    #[serde(rename = "errorCode")]
    error_code: Option<String>,
}

pub fn generate_irc_password() -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

impl IrcService {
    pub fn new<R: RedisPoolInterface>(arcadia: &Arcadia<R>) -> Result<Self> {
        let api_url = arcadia.ergo.api_url.as_ref().ok_or_else(|| {
            Error::IrcConfigurationError("ergo.api_url not configured".to_string())
        })?;
        let api_bearer_token = arcadia.ergo.api_bearer_token.as_ref().ok_or_else(|| {
            Error::IrcConfigurationError("ergo.api_bearer_token not configured".to_string())
        })?;

        Ok(IrcService {
            api_url: api_url.trim_end_matches('/').to_string(),
            api_bearer_token: api_bearer_token.clone(),
            client: arcadia.internal_http_client.clone(),
        })
    }

    pub async fn create_account(&self, username: &str, password: &str) -> Result<()> {
        let url = format!("{}/v1/saregister", self.api_url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_bearer_token)
            .json(&SaregisterRequest {
                account_name: username.to_string(),
                passphrase: password.to_string(),
            })
            .send()
            .await
            .map_err(|e| {
                Error::IrcAccountProvisioningError(format!(
                    "failed to reach Ergo API at {url}: {e}"
                ))
            })?;

        let status = response.status();
        let body: ErgoApiResponse = response.json().await.map_err(|e| {
            Error::IrcAccountProvisioningError(format!("failed to parse Ergo API response: {e}"))
        })?;

        if body.success {
            Ok(())
        } else {
            let error_code = body.error_code.unwrap_or_default();
            let error_message = body.error.unwrap_or_default();
            Err(Error::IrcAccountProvisioningError(format!(
                "SAREGISTER failed (HTTP {status}): {error_code} {error_message}"
            )))
        }
    }
}
