//! Alert delivery for stamon.
//!
//! Each kind of channel has its own config type, held by [`ChannelConfig`]
//! and stored as JSON. Adding a channel means adding a variant with its
//! config, a function that sends to it, and its arms in [`ChannelConfig`]'s
//! methods.

use std::{sync::OnceLock, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod email;
mod ntfy;
mod telegram;
mod webhook;

pub use email::{EmailConfig, EmailSecurity};
pub use ntfy::NtfyConfig;
pub use telegram::TelegramConfig;
pub use webhook::{ChatWebhookConfig, WebhookConfig};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ChannelConfig {
    /// POST the alert as JSON to any URL.
    Webhook(WebhookConfig),
    Slack(ChatWebhookConfig),
    Discord(ChatWebhookConfig),
    Telegram(TelegramConfig),
    Ntfy(NtfyConfig),
    Email(EmailConfig),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    Up,
    Down,
}

/// A change in a service's status, sent to its channels.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub service_id: u32,
    pub service_name: String,
    /// The URL or host being checked, if the service has one.
    pub target: Option<String>,
    pub status: AlertStatus,
    pub message: Option<String>,
    pub time: DateTime<Utc>,
}

impl Alert {
    pub fn title(&self) -> String {
        match self.status {
            AlertStatus::Down => format!("{} is down", self.service_name),
            AlertStatus::Up => format!("{} is back up", self.service_name),
        }
    }

    /// The details under the title, one per line.
    pub fn details(&self) -> String {
        let mut lines = Vec::new();
        if let Some(message) = &self.message {
            lines.push(message.clone());
        }
        if let Some(target) = &self.target {
            lines.push(format!("Target: {target}"));
        }
        lines.push(format!(
            "Time: {}",
            self.time.format("%Y-%m-%d %H:%M:%S UTC")
        ));
        lines.join("\n")
    }

    /// Title and details as one message for chat services.
    pub fn text(&self) -> String {
        let icon = match self.status {
            AlertStatus::Down => "🔴",
            AlertStatus::Up => "🟢",
        };
        format!("{icon} {}\n{}", self.title(), self.details())
    }
}

impl ChannelConfig {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            ChannelConfig::Webhook(config) => config.validate(),
            ChannelConfig::Slack(config) | ChannelConfig::Discord(config) => config.validate(),
            ChannelConfig::Telegram(config) => config.validate(),
            ChannelConfig::Ntfy(config) => config.validate(),
            ChannelConfig::Email(config) => config.validate(),
        }
    }

    pub async fn send(&self, alert: &Alert) -> Result<(), String> {
        match self {
            ChannelConfig::Webhook(config) => webhook::send(config, alert).await,
            ChannelConfig::Slack(config) => {
                webhook::send_chat(&config.webhook_url, "text", alert).await
            }
            ChannelConfig::Discord(config) => {
                webhook::send_chat(&config.webhook_url, "content", alert).await
            }
            ChannelConfig::Telegram(config) => telegram::send(config, alert).await,
            ChannelConfig::Ntfy(config) => ntfy::send(config, alert).await,
            ChannelConfig::Email(config) => email::send(config, alert).await,
        }
    }
}

fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("HTTP client should build")
    })
}

fn validate_http_url(name: &str, url: &str) -> Result<(), String> {
    let parsed = reqwest::Url::parse(url).map_err(|e| format!("invalid {name}: {e}"))?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(format!("{name} must start with http:// or https://"));
    }
    Ok(())
}

/// Turn a non-success response into an error that includes what the
/// provider said.
async fn ensure_success(provider: &str, res: reqwest::Response) -> Result<(), String> {
    let status = res.status();
    if status.is_success() {
        return Ok(());
    }
    let body = res.text().await.unwrap_or_default();
    let body: String = body.chars().take(300).collect();
    Err(format!("{provider} returned {status}: {body}"))
}

#[cfg(test)]
pub(crate) mod tests {
    use chrono::TimeZone;
    use serde_json::json;

    use super::*;

    pub fn alert(status: AlertStatus) -> Alert {
        Alert {
            service_id: 7,
            service_name: "API".into(),
            target: Some("https://api.example.com".into()),
            status,
            message: Some("Unexpected status code: 500".into()),
            time: Utc.with_ymd_and_hms(2026, 10, 9, 12, 0, 0).unwrap(),
        }
    }

    #[test]
    fn alert_text() {
        let down = alert(AlertStatus::Down);
        assert_eq!(down.title(), "API is down");
        assert_eq!(
            down.text(),
            "🔴 API is down\nUnexpected status code: 500\nTarget: https://api.example.com\nTime: 2026-10-09 12:00:00 UTC"
        );

        let up = Alert {
            message: None,
            target: None,
            ..alert(AlertStatus::Up)
        };
        assert_eq!(
            up.text(),
            "🟢 API is back up\nTime: 2026-10-09 12:00:00 UTC"
        );
    }

    #[test]
    fn config_json_is_tagged_by_type() {
        for config in [
            json!({ "type": "webhook", "url": "https://hooks.example.com/x" }),
            json!({ "type": "slack", "webhook_url": "https://hooks.slack.com/services/x" }),
            json!({ "type": "discord", "webhook_url": "https://discord.com/api/webhooks/x" }),
            json!({ "type": "telegram", "bot_token": "123:abc", "chat_id": "-100" }),
            json!({ "type": "ntfy", "topic": "stamon-alerts" }),
            json!({
                "type": "email",
                "host": "smtp.example.com",
                "from": "Stamon <stamon@example.com>",
                "to": ["ops@example.com"],
            }),
        ] {
            let parsed: ChannelConfig = serde_json::from_value(config.clone()).unwrap();
            assert!(parsed.validate().is_ok(), "{config}");
        }
        assert!(serde_json::from_value::<ChannelConfig>(json!({ "type": "pager" })).is_err());
    }
}
