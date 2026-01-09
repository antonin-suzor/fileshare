use anyhow::Context;
use reqwest::Client;
use serde_json::json;
use std::env;

pub struct DiscordService {}
impl DiscordService {
    fn get_webhook_url() -> anyhow::Result<String> {
        env::var("DISCORD_WEBHOOK_URL").context(
            "DISCORD_WEBHOOK_URL environment variable not set. Discord notifications will be disabled.",
        )
    }

    async fn send_webhook_message(content: &str) -> anyhow::Result<()> {
        let webhook_url = match Self::get_webhook_url() {
            Ok(url) => url,
            Err(_) => {
                // Discord webhook not configured, silently skip
                return Ok(());
            },
        };

        let client = Client::new();
        let payload = json!({
            "content": content
        });

        client
            .post(&webhook_url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send Discord webhook message")?;

        Ok(())
    }

    pub async fn notify_user_signup(email: &str) -> anyhow::Result<()> {
        let message = format!("New user signed up: {}", email);
        Self::send_webhook_message(&message).await
    }

    pub async fn notify_email_verified(email: &str) -> anyhow::Result<()> {
        let message = format!("Email verified: {}", email);
        Self::send_webhook_message(&message).await
    }

    pub async fn notify_upload_started(
        email: &str,
        filename: &str,
        presigned_get: &str,
    ) -> anyhow::Result<()> {
        let message = format!(
            "Upload started by {}: {} (`{}`)",
            email, filename, presigned_get
        );
        Self::send_webhook_message(&message).await
    }
}
