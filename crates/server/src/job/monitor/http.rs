use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use chrono::Utc;
use reqwest::{Client, header::CONTENT_TYPE};
use tokio::sync::broadcast::Sender;
use tracing::error;

use super::Service;
use crate::{
    models::log::{LogForCreate, Status},
    ws::{Event, Level, Notification},
};

fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

/// Whether `code` satisfies the service's expected code.
///
/// A single digit selects a whole class (`2` accepts any 2xx). Without an
/// expectation any 2xx is accepted.
fn status_matches(code: u16, expected: Option<u16>) -> bool {
    match expected {
        None | Some(0) => (200..300).contains(&code),
        Some(class @ 1..=5) => code / 100 == class,
        Some(expected) => code == expected,
    }
}

#[tracing::instrument(skip(svc, tx), fields(name = svc.name, url = svc.url))]
pub async fn get(svc: Service, tx: Sender<Event>) -> LogForCreate {
    let now = Instant::now();
    let time = Some(Utc::now());
    let result = |status, message| LogForCreate {
        status,
        service_id: svc.id,
        duration: now.elapsed().as_millis() as u32,
        time,
        message,
    };

    let request = match svc.payload.as_deref().filter(|p| !p.trim().is_empty()) {
        Some(body) => {
            let content_type = if serde_json::from_str::<serde_json::Value>(body).is_ok() {
                "application/json"
            } else {
                "text/plain"
            };
            client()
                .post(&svc.url)
                .header(CONTENT_TYPE, content_type)
                .body(body.to_owned())
        }
        None => client().get(&svc.url),
    };

    let res = match request
        .timeout(Duration::from_secs(svc.timeout.max(1).into()))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("Failed to get: {:?}", e);
            if e.is_connect()
                && let Err(e) = tx.send(Event::Notification(Notification {
                    message: format!("Error: {}", e),
                    title: "Network Error".to_string(),
                    level: Level::Error,
                }))
            {
                error!("Failed to send notification: {:?}", e);
            };
            return result(Status::Down, Some(format!("{e}")));
        }
    };

    let status = res.status();
    if !status_matches(status.as_u16(), svc.expected_code) {
        return result(
            Status::Down,
            Some(format!("Unexpected status code: {status}")),
        );
    }

    if let Some(json) = svc
        .expected_payload
        .as_deref()
        .filter(|p| !p.trim().is_empty())
    {
        let tmpl = match serde_json::from_str::<serde_json::Value>(json) {
            Ok(v) => v,
            Err(e) => {
                return result(
                    Status::Down,
                    Some(format!("Invalid expected payload template: {e}")),
                );
            }
        };
        let data = match res.json::<serde_json::Value>().await {
            Ok(v) => v,
            Err(e) => {
                return result(
                    Status::Down,
                    Some(format!("Failed to parse response JSON: {e}")),
                );
            }
        };
        if tmpl != data {
            return result(Status::Down, Some(format!("Expected: {json} Got: {data}")));
        }
    }

    result(Status::Up, None)
}

#[cfg(test)]
mod tests {
    use axum::{Json, Router, http::StatusCode, routing::get as get_route};
    use serde_json::json;
    use tokio::{net::TcpListener, sync::broadcast};

    use super::*;
    use crate::job::monitor::tests::service;

    async fn serve() -> String {
        let app = Router::new()
            .route("/ok", get_route(|| async { Json(json!({ "ok": true })) }))
            .route(
                "/error",
                get_route(|| async { StatusCode::INTERNAL_SERVER_ERROR }),
            )
            .route("/missing", get_route(|| async { StatusCode::NOT_FOUND }))
            .route(
                "/slow",
                get_route(|| async {
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    "late"
                }),
            );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        format!("http://{addr}")
    }

    async fn check(svc: Service) -> LogForCreate {
        let (tx, _rx) = broadcast::channel(10);
        get(svc, tx).await
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

    #[tokio::test]
    async fn success_is_up() {
        let base = serve().await;
        let log = check(service(&format!("{base}/ok"))).await;
        assert!(matches!(log.status, Status::Up), "{log:?}");
    }

    #[tokio::test]
    async fn server_error_is_down() {
        let base = serve().await;
        let log = check(service(&format!("{base}/error"))).await;
        assert!(matches!(log.status, Status::Down), "{log:?}");
        assert!(log.message.unwrap().contains("500"));
    }

    #[tokio::test]
    async fn expected_code_overrides_default() {
        let base = serve().await;
        let mut svc = service(&format!("{base}/missing"));
        svc.expected_code = Some(404);
        let log = check(svc).await;
        assert!(matches!(log.status, Status::Up), "{log:?}");
    }

    #[tokio::test]
    async fn expected_payload_is_compared() {
        let base = serve().await;
        let mut svc = service(&format!("{base}/ok"));
        svc.expected_payload = Some(r#"{"ok": true}"#.into());
        assert!(matches!(check(svc.clone()).await.status, Status::Up));

        svc.expected_payload = Some(r#"{"ok": false}"#.into());
        assert!(matches!(check(svc.clone()).await.status, Status::Down));

        // An empty template from the form means no payload check
        svc.expected_payload = Some(String::new());
        assert!(matches!(check(svc).await.status, Status::Up));
    }

    #[tokio::test]
    async fn timeout_is_down() {
        let base = serve().await;
        let mut svc = service(&format!("{base}/slow"));
        svc.timeout = 1;
        let log = check(svc).await;
        assert!(matches!(log.status, Status::Down), "{log:?}");
    }
}
