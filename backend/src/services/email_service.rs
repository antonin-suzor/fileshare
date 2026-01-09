use anyhow::Context;
use lettre::transport::smtp::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};
use std::env;

use crate::entities::{User, Verification};

pub struct EmailService {}
impl EmailService {
    pub fn get_smtp_transport() -> anyhow::Result<SmtpTransport> {
        let mail_user = env::var("MAIL_USER")?;
        let mail_password = env::var("MAIL_PASSWORD")?;
        let creds = Credentials::new(mail_user, mail_password);
        Ok(if cfg!(debug_assertions) {
            SmtpTransport::builder_dangerous("localhost")
                .port(1025)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::relay("smtp.gmail.com")?
                .credentials(creds)
                .build()
        })
    }

    /// Sends a verification email to the user
    pub async fn send_verification_email(
        user: &User,
        verification: &Verification,
    ) -> anyhow::Result<()> {
        let web_host = env::var("WEB_HOST")?;
        let verification_link = format!("{}/account/verify-email?id={}", web_host, verification.id);
        let from_email = env::var("MAIL_FROM")?;

        let email = Message::builder()
            .from(
                from_email
                    .parse()
                    .context("Failed to parse FROM email address")?,
            )
            .to(user.email.parse().context("Failed to parse TO email address")?)
            .subject("Welcome to FileShare - Verify Your Email")
            .body(format!(
                "Welcome to FileShare!\n\nPlease verify your email by clicking the link below:\n\n{}\n\nIf you didn't create this account, you can safely ignore this email.",
                verification_link
            ))
            .context("Failed to build email message")?;

        let transport: SmtpTransport = Self::get_smtp_transport()?;
        transport.send(&email).context("Failed to send email")?;

        Ok(())
    }
}
