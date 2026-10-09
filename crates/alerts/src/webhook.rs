use std::collections::BTreeMap;

use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Alert, client, ensure_success, validate_http_url};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
}

/// Slack and Discord incoming webhooks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatWebhookConfig {
    pub webhook_url: String,
}

impl WebhookConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_http_url("url", &self.url)?;
        for (name, value) in &self.headers {
            HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| format!("invalid header name: {name}"))?;
            HeaderValue::from_str(value).map_err(|_| format!("invalid value for header {name}"))?;
        }
        Ok(())
    }
}

impl ChatWebhookConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_http_url("webhook_url", &self.webhook_url)
    }
}

/// POST the alert as JSON, with a ready-made `title` and `text`.
pub(crate) async fn send(config: &WebhookConfig, alert: &Alert) -> Result<(), String> {
    let mut body = serde_json::to_value(alert).map_err(|e| e.to_string())?;
    body["title"] = json!(alert.title());
    body["text"] = json!(alert.text());

    let mut request = client().post(&config.url).json(&body);
    for (name, value) in &config.headers {
        request = request.header(name, value);
    }
    let res = request
        .send()
        .await
        .map_err(|e| format!("Webhook request failed: {}", e.without_url()))?;
    ensure_success("Webhook", res).await
}

/// POST `{ <field>: text }`, the shape Slack (`text`) and Discord
/// (`content`) webhooks expect.
pub(crate) async fn send_chat(url: &str, field: &str, alert: &Alert) -> Result<(), String> {
    let res = client()
        .post(url)
        .json(&json!({ field: alert.text() }))
        .send()
        .await
        .map_err(|e| format!("Webhook request failed: {}", e.without_url()))?;
    ensure_success("Webhook", res).await
}

#[cfg(test)]
pub(crate) mod tests {
    use std::sync::{Arc, Mutex};

    use axum::{
        Json, Router,
        extract::State,
        http::{HeaderMap, StatusCode},
        routing::post,
    };
    use serde_json::Value;
    use tokio::net::TcpListener;

    use super::*;
    use crate::{AlertStatus, ChannelConfig, tests::alert};

    /// A received request: its headers and JSON body.
    pub type Received = Arc<Mutex<Vec<(HeaderMap, Value)>>>;

    /// Accept JSON POSTs on any path, replying with `status`.
    pub async fn serve(status: StatusCode) -> (String, Received) {
        let received = Received::default();
        let app = Router::new()
            .route(
                "/{*path}",
                post(
                    move |State(received): State<Received>,
                          headers: HeaderMap,
                          Json(body): Json<Value>| async move {
                        received.lock().unwrap().push((headers, body));
                        (status, "provider said no")
                    },
                ),
            )
            .with_state(received.clone());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        (format!("http://{addr}"), received)
    }

    #[tokio::test]
    async fn webhook_posts_alert_json_with_headers() {
        let (base, received) = serve(StatusCode::OK).await;
        let config = ChannelConfig::Webhook(WebhookConfig {
            url: format!("{base}/hook"),
            headers: BTreeMap::from([("X-Secret".into(), "s3cret".into())]),
        });
        config.send(&alert(AlertStatus::Down)).await.unwrap();

        let received = received.lock().unwrap();
        let (headers, body) = &received[0];
        assert_eq!(headers["x-secret"], "s3cret");
        assert_eq!(body["status"], "down");
        assert_eq!(body["service_name"], "API");
        assert_eq!(body["title"], "API is down");
        assert!(body["text"].as_str().unwrap().starts_with("🔴 API is down"));
    }

    #[tokio::test]
    async fn slack_and_discord_use_their_fields() {
        let (base, received) = serve(StatusCode::NO_CONTENT).await;
        let chat = ChatWebhookConfig {
            webhook_url: format!("{base}/chat"),
        };
        ChannelConfig::Slack(chat.clone())
            .send(&alert(AlertStatus::Up))
            .await
            .unwrap();
        ChannelConfig::Discord(chat)
            .send(&alert(AlertStatus::Up))
            .await
            .unwrap();

        let received = received.lock().unwrap();
        assert!(received[0].1["text"].as_str().unwrap().contains("back up"));
        assert!(
            received[1].1["content"]
                .as_str()
                .unwrap()
                .contains("back up")
        );
    }

    #[tokio::test]
    async fn provider_errors_are_reported() {
        let (base, _) = serve(StatusCode::FORBIDDEN).await;
        let config = ChannelConfig::Slack(ChatWebhookConfig {
            webhook_url: format!("{base}/chat"),
        });
        let err = config.send(&alert(AlertStatus::Down)).await.unwrap_err();
        assert_eq!(err, "Webhook returned 403 Forbidden: provider said no");
    }

    #[test]
    fn validation() {
        let webhook = |url: &str| WebhookConfig {
            url: url.into(),
            headers: BTreeMap::new(),
        };
        assert!(webhook("https://example.com/hook").validate().is_ok());
        assert!(webhook("example.com/hook").validate().is_err());
        let mut bad_header = webhook("https://example.com");
        bad_header.headers.insert("bad header".into(), "x".into());
        assert!(bad_header.validate().is_err());
        assert!(
            ChatWebhookConfig {
                webhook_url: "ftp://x".into()
            }
            .validate()
            .is_err()
        );
    }
}
