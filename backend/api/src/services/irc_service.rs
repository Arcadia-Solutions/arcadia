use crate::Arcadia;
use arcadia_common::error::{Error, Result};
use arcadia_storage::redis::RedisPoolInterface;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct IrcService {
    irc_host: String,
    irc_port: u16,
    oper_name: String,
    oper_password: String,
}

impl IrcService {
    pub fn new<R: RedisPoolInterface>(config: &Arcadia<R>) -> Result<Self> {
        let irc_host = config.ergo.irc_host.as_ref().ok_or_else(|| {
            Error::IrcConfigurationError("ERGO_IRC_HOST not configured".to_string())
        })?;
        let oper_name = config.ergo.oper_name.as_ref().ok_or_else(|| {
            Error::IrcConfigurationError("ERGO_OPER_NAME not configured".to_string())
        })?;
        let oper_password = config.ergo.oper_password.as_ref().ok_or_else(|| {
            Error::IrcConfigurationError("ERGO_OPER_PASSWORD not configured".to_string())
        })?;

        Ok(IrcService {
            irc_host: irc_host.clone(),
            irc_port: config.ergo.irc_port,
            oper_name: oper_name.clone(),
            oper_password: oper_password.clone(),
        })
    }

    pub async fn create_account(&self, username: &str, password: &str) -> Result<()> {
        let address = format!("{}:{}", self.irc_host, self.irc_port);
        let stream = TcpStream::connect(&address).await.map_err(|e| {
            Error::IrcAccountProvisioningError(format!(
                "failed to connect to Ergo at {address}: {e}"
            ))
        })?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Register as a bot connection
        self.send_line(&mut writer, "NICK arcadia_bot").await?;
        self.send_line(&mut writer, "USER arcadia_bot 0 * :Arcadia Bot")
            .await?;

        // Wait for welcome (001) or error
        self.wait_for_reply(&mut reader, "001").await?;

        // Authenticate as operator
        self.send_line(
            &mut writer,
            &format!("OPER {} {}", self.oper_name, self.oper_password),
        )
        .await?;

        // Wait for RPL_YOUREOPER (381) or error
        self.wait_for_reply(&mut reader, "381").await?;

        // Register the account using NickServ SAREGISTER
        self.send_line(&mut writer, &format!("NS SAREGISTER {username} {password}"))
            .await?;

        // Read the NickServ response (NOTICE from NickServ)
        let response = self.wait_for_nickserv_reply(&mut reader).await?;

        // Clean up
        let _ = self.send_line(&mut writer, "QUIT").await;

        // Check if SAREGISTER succeeded
        if response.contains("successfully registered")
            || response.contains("Account created")
            || response.contains("account created")
        {
            Ok(())
        } else {
            Err(Error::IrcAccountProvisioningError(format!(
                "SAREGISTER failed: {response}"
            )))
        }
    }

    async fn send_line(
        &self,
        writer: &mut tokio::net::tcp::OwnedWriteHalf,
        line: &str,
    ) -> Result<()> {
        writer
            .write_all(format!("{line}\r\n").as_bytes())
            .await
            .map_err(|e| {
                Error::IrcAccountProvisioningError(format!("failed to send IRC command: {e}"))
            })
    }

    async fn wait_for_reply(
        &self,
        reader: &mut BufReader<tokio::net::tcp::OwnedReadHalf>,
        numeric: &str,
    ) -> Result<String> {
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                reader.read_line(&mut line),
            )
            .await
            .map_err(|_| {
                Error::IrcAccountProvisioningError(format!(
                    "timeout waiting for IRC reply {numeric}"
                ))
            })?
            .map_err(|e| {
                Error::IrcAccountProvisioningError(format!("failed to read IRC reply: {e}"))
            })?;

            if bytes_read == 0 {
                return Err(Error::IrcAccountProvisioningError(
                    "connection closed by Ergo".to_string(),
                ));
            }

            // Respond to PING to keep the connection alive
            if line.starts_with("PING") {
                let token = line.trim_start_matches("PING ").trim();
                // We don't have the writer here, but PINGs during initial
                // connection are rare; if needed we can restructure.
                // For now just continue reading.
                log::debug!("Received PING {token} during IRC handshake");
            }

            // Check for error numerics
            if line.contains(" 464 ") || line.contains(" 491 ") {
                return Err(Error::IrcAccountProvisioningError(format!(
                    "IRC authentication failed: {line}"
                )));
            }

            if line.contains(&format!(" {numeric} ")) {
                return Ok(line);
            }
        }
    }

    async fn wait_for_nickserv_reply(
        &self,
        reader: &mut BufReader<tokio::net::tcp::OwnedReadHalf>,
    ) -> Result<String> {
        let mut line = String::new();
        loop {
            line.clear();
            let bytes_read = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                reader.read_line(&mut line),
            )
            .await
            .map_err(|_| {
                Error::IrcAccountProvisioningError("timeout waiting for NickServ reply".to_string())
            })?
            .map_err(|e| {
                Error::IrcAccountProvisioningError(format!("failed to read NickServ reply: {e}"))
            })?;

            if bytes_read == 0 {
                return Err(Error::IrcAccountProvisioningError(
                    "connection closed by Ergo".to_string(),
                ));
            }

            // NickServ replies come as NOTICE from NickServ
            let line_lower = line.to_lowercase();
            if line_lower.contains("nickserv") && line.contains("NOTICE") {
                return Ok(line);
            }
        }
    }
}
