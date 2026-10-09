use std::{collections::BTreeMap, sync::OnceLock, time::Instant};

use reqwest::{
    Client, Method, Url,
    header::{CONTENT_TYPE, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::CheckOutcome;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpConfig {
    pub url: String,
    #[serde(default)]
    pub method: HttpMethod,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(default)]
    pub body: Option<String>,
    /// Expected status code, or a single digit for a whole class (`2`
    /// accepts any 2xx). Without one any 2xx is accepted.
    #[serde(default)]
    pub expected_code: Option<u16>,
    /// JSON the response body must equal.
    #[serde(default)]
    pub expected_payload: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    #[default]
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
    Options,
}

impl From<HttpMethod> for Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::Get => Method::GET,
            HttpMethod::Head => Method::HEAD,
            HttpMethod::Post => Method::POST,
            HttpMethod::Put => Method::PUT,
            HttpMethod::Patch => Method::PATCH,
            HttpMethod::Delete => Method::DELETE,
            HttpMethod::Options => Method::OPTIONS,
        }
    }
}

fn non_empty(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|v| !v.trim().is_empty())
}

impl HttpConfig {
    #[cfg(test)]
    pub(crate) fn get(url: &str) -> Self {
        Self {
            url: url.into(),
            method: HttpMethod::Get,
            headers: BTreeMap::new(),
            body: None,
            expected_code: None,
            expected_payload: None,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let url = Url::parse(&self.url).map_err(|e| format!("invalid url: {e}"))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err("url must start with http:// or https://".into());
        }
        for (name, value) in &self.headers {
            HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| format!("invalid header name: {name}"))?;
            HeaderValue::from_str(value).map_err(|_| format!("invalid value for header {name}"))?;
        }
        if let Some(code) = self.expected_code
            && !matches!(code, 0..=5 | 100..=599)
        {
            return Err(
                "expected_code must be a status class (1-5) or a status code (100-599)".into(),
            );
        }
        if let Some(payload) = non_empty(&self.expected_payload) {
            serde_json::from_str::<serde_json::Value>(payload)
                .map_err(|e| format!("expected_payload must be valid JSON: {e}"))?;
        }
        Ok(())
    }
}

fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

fn status_matches(code: u16, expected: Option<u16>) -> bool {
    match expected {
        None | Some(0) => (200..300).contains(&code),
        Some(class @ 1..=5) => code / 100 == class,
        Some(expected) => code == expected,
    }
}

pub(crate) async fn check(config: &HttpConfig) -> CheckOutcome {
    let start = Instant::now();

    let mut request = client().request(config.method.into(), &config.url);
    for (name, value) in &config.headers {
        request = request.header(name, value);
    }
    if let Some(body) = non_empty(&config.body) {
        let has_content_type = config
            .headers
            .keys()
            .any(|name| name.eq_ignore_ascii_case(CONTENT_TYPE.as_str()));
        if !has_content_type {
            let content_type = if serde_json::from_str::<serde_json::Value>(body).is_ok() {
                "application/json"
            } else {
                "text/plain"
            };
            request = request.header(CONTENT_TYPE, content_type);
        }
        request = request.body(body.to_owned());
    }

    let res = match request.send().await {
        Ok(res) => res,
        Err(e) => return CheckOutcome::down(start.elapsed(), e.to_string()),
    };

    let status = res.status();
    if !status_matches(status.as_u16(), config.expected_code) {
        return CheckOutcome::down(start.elapsed(), format!("Unexpected status code: {status}"));
    }

    if let Some(expected) = non_empty(&config.expected_payload) {
        let expected = match serde_json::from_str::<serde_json::Value>(expected) {
            Ok(v) => v,
            Err(e) => {
                return CheckOutcome::error(format!("Invalid expected payload template: {e}"));
            }
        };
        let data = match res.json::<serde_json::Value>().await {
            Ok(v) => v,
            Err(e) => {
                return CheckOutcome::down(
                    start.elapsed(),
                    format!("Failed to parse response JSON: {e}"),
                );
            }
        };
        if expected != data {
            return CheckOutcome::down(
                start.elapsed(),
                format!("Expected: {expected} Got: {data}"),
            );
        }
    }

    CheckOutcome::up(start.elapsed())
}

#[cfg(test)]
mod tests {
    use axum::{
        Json, Router,
        http::{HeaderMap, StatusCode},
        routing::{get, post},
    };
    use serde_json::json;
    use tokio::net::TcpListener;

    use super::*;
    use crate::CheckStatus;

    async fn serve() -> String {
        let app = Router::new()
            .route("/ok", get(|| async { Json(json!({ "ok": true })) }))
            .route(
                "/error",
                get(|| async { StatusCode::INTERNAL_SERVER_ERROR }),
            )
            .route("/missing", get(|| async { StatusCode::NOT_FOUND }))
            .route(
                "/echo",
                post(|headers: HeaderMap, body: String| async move {
                    let header = |name: &str| {
                        headers
                            .get(name)
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or_default()
                            .to_owned()
                    };
                    Json(json!({
                        "body": body,
                        "content_type": header("content-type"),
                        "token": header("x-token"),
                    }))
                }),
            );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        format!("http://{addr}")
    }

    #[test]
    fn expected_code_matching() {
        assert!(status_matches(204, None));
        assert!(!status_matches(500, None));
        assert!(!status_matches(301, Some(0)));
        assert!(status_matches(201, Some(2)));
        assert!(!status_matches(404, Some(2)));
        assert!(status_matches(404, Some(4)));
        assert!(status_matches(404, Some(404)));
        assert!(!status_matches(200, Some(404)));
    }

    #[test]
    fn validation() {
        assert!(HttpConfig::get("https://example.com").validate().is_ok());
        assert!(HttpConfig::get("example.com").validate().is_err());
        assert!(HttpConfig::get("ftp://example.com").validate().is_err());

        let mut config = HttpConfig::get("https://example.com");
        config.expected_code = Some(42);
        assert!(config.validate().is_err());

        let mut config = HttpConfig::get("https://example.com");
        config.expected_payload = Some("{not json".into());
        assert!(config.validate().is_err());
        config.expected_payload = Some(" ".into());
        assert!(config.validate().is_ok());

        let mut config = HttpConfig::get("https://example.com");
        config.headers.insert("bad header".into(), "x".into());
        assert!(config.validate().is_err());
    }

    #[tokio::test]
    async fn success_is_up() {
        let base = serve().await;
        let outcome = check(&HttpConfig::get(&format!("{base}/ok"))).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
    }

    #[tokio::test]
    async fn server_error_is_down() {
        let base = serve().await;
        let outcome = check(&HttpConfig::get(&format!("{base}/error"))).await;
        assert_eq!(outcome.status, CheckStatus::Down, "{outcome:?}");
        assert!(outcome.message.unwrap().contains("500"));
    }

    #[tokio::test]
    async fn connection_refused_is_down() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        drop(listener);
        let outcome = check(&HttpConfig::get(&url)).await;
        assert_eq!(outcome.status, CheckStatus::Down, "{outcome:?}");
    }

    #[tokio::test]
    async fn expected_code_overrides_default() {
        let base = serve().await;
        let mut config = HttpConfig::get(&format!("{base}/missing"));
        config.expected_code = Some(404);
        assert_eq!(check(&config).await.status, CheckStatus::Up);
    }

    #[tokio::test]
    async fn expected_payload_is_compared() {
        let base = serve().await;
        let mut config = HttpConfig::get(&format!("{base}/ok"));
        config.expected_payload = Some(r#"{"ok": true}"#.into());
        assert_eq!(check(&config).await.status, CheckStatus::Up);

        config.expected_payload = Some(r#"{"ok": false}"#.into());
        assert_eq!(check(&config).await.status, CheckStatus::Down);

        config.expected_payload = Some(String::new());
        assert_eq!(check(&config).await.status, CheckStatus::Up);
    }

    #[tokio::test]
    async fn sends_method_headers_and_body() {
        let base = serve().await;
        let mut config = HttpConfig::get(&format!("{base}/echo"));
        config.method = HttpMethod::Post;
        config.headers.insert("X-Token".into(), "secret".into());
        config.body = Some(r#"{"ping":1}"#.into());
        config.expected_payload = Some(
            json!({
                "body": r#"{"ping":1}"#,
                "content_type": "application/json",
                "token": "secret",
            })
            .to_string(),
        );
        let outcome = check(&config).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");

        // An explicit content type is kept.
        config
            .headers
            .insert("content-type".into(), "text/x-custom".into());
        config.expected_payload = Some(
            json!({
                "body": r#"{"ping":1}"#,
                "content_type": "text/x-custom",
                "token": "secret",
            })
            .to_string(),
        );
        let outcome = check(&config).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
    }
}
