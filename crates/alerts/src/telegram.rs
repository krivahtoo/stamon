use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Alert, client, ensure_success};

const API_URL: &str = "https://api.telegram.org";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelegramConfig {
    /// From @BotFather, e.g. `123456:ABC-DEF...`.
    pub bot_token: String,
    /// The user, group or channel to message.
    pub chat_id: String,
}

impl TelegramConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.bot_token.contains(':') || self.bot_token.contains('/') {
            return Err("bot_token should look like 123456:ABC-DEF".into());
        }
        if self.chat_id.trim().is_empty() {
            return Err("chat_id must not be empty".into());
        }
        Ok(())
    }
}

pub(crate) async fn send(config: &TelegramConfig, alert: &Alert) -> Result<(), String> {
    send_via(API_URL, config, alert).await
}

async fn send_via(api_url: &str, config: &TelegramConfig, alert: &Alert) -> Result<(), String> {
    let res = client()
        .post(format!("{api_url}/bot{}/sendMessage", config.bot_token))
        .json(&json!({ "chat_id": config.chat_id, "text": alert.text() }))
        .send()
        .await
        // Leave out the URL: it contains the bot token.
        .map_err(|e| format!("Telegram request failed: {}", e.without_url()))?;
    ensure_success("Telegram", res).await
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::*;
    use crate::{AlertStatus, tests::alert, webhook::tests::serve};

    #[tokio::test]
    async fn sends_message_to_chat() {
        let (base, received) = serve(StatusCode::OK).await;
        let config = TelegramConfig {
            bot_token: "123:abc".into(),
            chat_id: "-100200".into(),
        };
        send_via(&base, &config, &alert(AlertStatus::Down))
            .await
            .unwrap();

        let received = received.lock().unwrap();
        let (_, body) = &received[0];
        assert_eq!(body["chat_id"], "-100200");
        assert!(body["text"].as_str().unwrap().starts_with("🔴 API is down"));
    }

    #[test]
    fn validation() {
        let config = |token: &str, chat: &str| TelegramConfig {
            bot_token: token.into(),
            chat_id: chat.into(),
        };
        assert!(config("123:abc", "42").validate().is_ok());
        assert!(config("no-colon", "42").validate().is_err());
        assert!(config("123:abc/../x", "42").validate().is_err());
        assert!(config("123:abc", " ").validate().is_err());
    }
}
