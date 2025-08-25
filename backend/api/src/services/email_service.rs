use crate::Arcadia;
use arcadia_common::error::{Error, Result};
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
    from_name: String,
    tracker_name: String,
    frontend_url: String,
}

impl EmailService {
    pub fn new(config: &Arcadia) -> Result<Self> {
        // Check if all required SMTP configuration is present
        let smtp_host = config.smtp.host.as_ref().ok_or_else(|| {
            Error::EmailConfigurationError("SMTP_HOST not configured".to_string())
        })?;
        let smtp_port = config.smtp.port.ok_or_else(|| {
            Error::EmailConfigurationError("SMTP_PORT not configured".to_string())
        })?;
        let smtp_username = config.smtp.username.as_ref().ok_or_else(|| {
            Error::EmailConfigurationError("SMTP_USERNAME not configured".to_string())
        })?;
        let smtp_password = config.smtp.password.as_ref().ok_or_else(|| {
            Error::EmailConfigurationError("SMTP_PASSWORD not configured".to_string())
        })?;
        let smtp_from_email = config.smtp.from_email.as_ref().ok_or_else(|| {
            Error::EmailConfigurationError("SMTP_FROM_EMAIL not configured".to_string())
        })?;
        let smtp_from_name = config.smtp.from_name.as_ref().ok_or_else(|| {
            Error::EmailConfigurationError("SMTP_FROM_NAME not configured".to_string())
        })?;

        let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());

        let mailer = SmtpTransport::relay(smtp_host)
            .map_err(|e| Error::EmailConfigurationError(e.to_string()))?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(EmailService {
            mailer,
            from_email: smtp_from_email.clone(),
            from_name: smtp_from_name.clone(),
            tracker_name: config.tracker.name.clone(),
            frontend_url: config.frontend_url.to_string(),
        })
    }

    pub async fn send_registration_email(&self, user_email: &str, username: &str) -> Result<()> {
        let subject = format!("Welcome to {}!", self.tracker_name);
        let body = format!(
            "Hello {},\n\n\
            Welcome to {}! Your account has been successfully created.\n\n\
            You can now log in and start using our tracker at: {}\n\n\
            Best regards,\n\
            The {} Team",
            username, self.tracker_name, self.frontend_url, self.tracker_name
        );

        self.send_email(user_email, &subject, &body).await
    }

    pub async fn send_invitation_email(
        &self,
        recipient_email: &str,
        sender_username: &str,
        invitation_key: &str,
        message: &str,
    ) -> Result<()> {
        let subject = format!("You've been invited to join {}!", self.tracker_name);
        let invitation_url = format!(
            "{}/register?invitation_key={}",
            self.frontend_url.trim_end_matches('/'),
            invitation_key
        );

        let body = format!(
            "Hello,\n\n\
            {} has invited you to join {}!\n\n\
            Personal message from {}:\n\
            {}\n\n\
            To accept this invitation and create your account, please click the link below:\n\
            {}\n\n\
            This invitation will expire in 7 days.\n\n\
            Best regards,\n\
            The {} Team",
            sender_username,
            self.tracker_name,
            sender_username,
            message,
            invitation_url,
            self.tracker_name
        );

        self.send_email(recipient_email, &subject, &body).await
    }

    async fn send_email(&self, to_email: &str, subject: &str, body: &str) -> Result<()> {
        let from_mailbox = Mailbox::new(
            Some(self.from_name.clone()),
            self.from_email
                .parse()
                .map_err(|e| Error::EmailConfigurationError(format!("Invalid from email: {e}")))?,
        );

        let to_mailbox = Mailbox::new(
            None,
            to_email
                .parse()
                .map_err(|e| Error::EmailSendError(format!("Invalid recipient email: {e}")))?,
        );

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| Error::EmailSendError(format!("Failed to build email: {e}")))?;

        // Send the email in a blocking task to avoid blocking the async runtime
        let mailer = self.mailer.clone();
        tokio::task::spawn_blocking(move || mailer.send(&email))
            .await
            .map_err(|e| Error::EmailSendError(format!("Task join error: {e}")))?
            .map_err(|e| Error::EmailSendError(format!("Failed to send email: {e}")))?;

        Ok(())
    }
}
