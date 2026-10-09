use serde::{Deserialize, Serialize};

use crate::{Alert, AlertStatus, client, ensure_success, validate_http_url};

fn default_server_url() -> String {
    "https://ntfy.sh".into()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NtfyConfig {
    #[serde(default = "default_server_url")]
    pub server_url: String,
    pub topic: String,
    /// For servers or topics that need authentication.
    #[serde(default)]
    pub access_token: Option<String>,
}

impl NtfyConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_http_url("server_url", &self.server_url)?;
        if self.topic.is_empty()
            || !self
                .topic
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err("topic may only contain letters, digits, - and _".into());
        }
        Ok(())
    }
}

pub(crate) async fn send(config: &NtfyConfig, alert: &Alert) -> Result<(), String> {
    let (priority, tags) = match alert.status {
        AlertStatus::Down => ("high", "red_circle"),
        AlertStatus::Up => ("default", "green_circle"),
    };
    let mut request = client()
        .post(format!(
            "{}/{}",
            config.server_url.trim_end_matches('/'),
            config.topic
        ))
        .header("Title", alert.title())
        .header("Priority", priority)
        .header("Tags", tags)
        .body(alert.details());
    if let Some(token) = config.access_token.as_deref().filter(|t| !t.is_empty()) {
        request = request.bearer_auth(token);
    }
    let res = request
        .send()
        .await
        .map_err(|e| format!("ntfy request failed: {}", e.without_url()))?;
    ensure_success("ntfy", res).await
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use axum::{Router, extract::State, http::HeaderMap, routing::post};
    use tokio::net::TcpListener;

    use super::*;
    use crate::tests::alert;

    #[tokio::test]
    async fn publishes_to_topic_with_title_and_priority() {
        type Received = Arc<Mutex<Vec<(String, HeaderMap, String)>>>;
        let received = Received::default();
        let app = Router::new()
            .route(
                "/{topic}",
                post(
                    |State(received): State<Received>,
                     axum::extract::Path(topic): axum::extract::Path<String>,
                     headers: HeaderMap,
                     body: String| async move {
                        received.lock().unwrap().push((topic, headers, body));
                    },
                ),
            )
            .with_state(received.clone());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

        let config = NtfyConfig {
            server_url: format!("http://{addr}/"),
            topic: "stamon-alerts".into(),
            access_token: Some("tk_secret".into()),
        };
        send(&config, &alert(AlertStatus::Down)).await.unwrap();

        let received = received.lock().unwrap();
        let (topic, headers, body) = &received[0];
        assert_eq!(topic, "stamon-alerts");
        assert_eq!(headers["title"], "API is down");
        assert_eq!(headers["priority"], "high");
        assert_eq!(headers["authorization"], "Bearer tk_secret");
        assert!(body.starts_with("Unexpected status code: 500"));
    }

    #[test]
    fn validation() {
        let config = |topic: &str| NtfyConfig {
            server_url: default_server_url(),
            topic: topic.into(),
            access_token: None,
        };
        assert!(config("stamon_alerts-1").validate().is_ok());
        assert!(config("").validate().is_err());
        assert!(config("a/b").validate().is_err());
    }
}
