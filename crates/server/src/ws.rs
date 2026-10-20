use std::{
    net::SocketAddr,
    ops::ControlFlow,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::{
        ConnectInfo, State, WebSocketUpgrade,
        ws::{CloseFrame, Message, WebSocket, close_code},
    },
    http::{
        HeaderMap, StatusCode,
        header::{HOST, ORIGIN},
    },
    response::{IntoResponse, Json, Response},
};
use axum_extra::{TypedHeader, headers::UserAgent};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::json;
use tracing::{debug, info, warn};

use crate::{
    AppState,
    auth::{AuthError, Claims, ViewerClaims},
    models::log::LogForCreate,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    //Message,
    Success,
    Error,
    Info,
    Warning,
}

#[derive(Debug, Clone, Serialize)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub level: Level,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum Event {
    /// Send a log entry
    Log(LogForCreate),
    /// Used to send notification that will show as popup
    Notification(Notification),
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
/// Like the API, it needs a viewer's token, which browsers send as the `token` cookie.
pub async fn ws_handler(
    ViewerClaims(claims): ViewerClaims,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> Response {
    // Browsers send the cookie from any page on the same site, so only the
    // dashboard's own origin may open the socket.
    if !same_origin(&headers) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Origin not allowed" })),
        )
            .into_response();
    }
    // The token check allows some leeway after expiry; the socket must not.
    let expires_in = time_left(&claims, SystemTime::now());
    if expires_in.is_zero() {
        return AuthError::InvalidToken.into_response();
    }
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state, expires_in))
        .into_response()
}

/// Whether the handshake comes from the page's own origin. Requests without
/// `Origin` don't come from a browser page, so they can't ride on its cookie.
/// Browsers can't set `X-Forwarded-Host` on a WebSocket handshake, so it is
/// safe to accept for proxies that rewrite `Host`.
fn same_origin(headers: &HeaderMap) -> bool {
    let Some(origin) = headers.get(ORIGIN) else {
        return true;
    };
    let Some((_, origin)) = origin.to_str().ok().and_then(|o| o.split_once("://")) else {
        return false;
    };
    [HOST.as_str(), "x-forwarded-host"]
        .into_iter()
        .filter_map(|name| headers.get(name)?.to_str().ok())
        .any(|host| host.eq_ignore_ascii_case(origin))
}

/// How long to wait for the close frame to go out once the token expires.
const CLOSE_TIMEOUT: Duration = Duration::from_secs(5);

/// How long until the token expires, so the socket doesn't outlive the sign-in.
fn time_left(claims: &Claims, now: SystemTime) -> Duration {
    let now = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    Duration::from_secs((claims.expiry as u64).saturating_sub(now))
}

/// Actual websocket statemachine (one will be spawned per connection)
#[tracing::instrument(name = "ws", skip(socket, state))]
async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    state: AppState,
    expires_in: Duration,
) {
    // send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket
        .send(Message::Ping(vec![1, 2, 3].into()))
        .await
        .is_ok()
    {
        debug!("Pinged {who}...");
    } else {
        warn!("Could not send ping {who}!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receive) = socket.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    // Close the socket when the token expires; the client reconnects with a fresh one.
    let mut send_task = tokio::spawn(async move {
        let expired = tokio::time::sleep(expires_in);
        tokio::pin!(expired);
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    let Ok(msg) = msg else { break };
                    let text = Message::Text(serde_json::to_string(&msg).unwrap().into());
                    // Expiry must win even while a slow client holds up the send.
                    tokio::select! {
                        sent = sender.send(text) => {
                            // In any websocket error, break loop.
                            if sent.is_err() {
                                break;
                            }
                        }
                        _ = &mut expired => {
                            debug!("Token expired mid-send, dropping {who}");
                            break;
                        }
                    }
                }
                _ = &mut expired => {
                    debug!("Token expired, closing {who}");
                    let close = Message::Close(Some(CloseFrame {
                        code: close_code::POLICY,
                        reason: "Token expired".into(),
                    }));
                    // Don't let a client that stopped reading keep the socket open.
                    let _ = tokio::time::timeout(CLOSE_TIMEOUT, sender.send(close)).await;
                    break;
                }
            }
        }
    });

    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receive.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            match process_message(msg, who) {
                ControlFlow::Break(_) => break,
                ControlFlow::Continue(_) => (),
            }
        }
        debug!("Processed {cnt} messages");
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };

    // returning from the handler closes the websocket connection
    info!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
#[tracing::instrument(skip(msg))]
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            debug!("received str: {t:?}");
        }
        Message::Close(cf) => {
            debug!("close received {:?}", cf);
            return ControlFlow::Break(());
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec.
        m => debug!("Received {m:?}"),
    }
    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::UserRole;

    fn claims(expiry: usize) -> Claims {
        Claims {
            user_id: 1,
            role: UserRole::Viewer,
            issued_at: 0,
            expiry,
        }
    }

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        pairs
            .iter()
            .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
            .collect()
    }

    #[test]
    fn same_origin_checks_host() {
        let ok = |pairs| same_origin(&headers(pairs));
        assert!(ok(&[("host", "status.example.com")]), "no origin");
        assert!(ok(&[
            ("host", "status.example.com"),
            ("origin", "https://status.example.com"),
        ]));
        assert!(ok(&[
            ("host", "localhost:5173"),
            ("origin", "http://localhost:5173"),
        ]));
        assert!(ok(&[
            ("host", "127.0.0.1:3000"),
            ("x-forwarded-host", "status.example.com"),
            ("origin", "https://status.example.com"),
        ]));
        assert!(!ok(&[
            ("host", "status.example.com"),
            ("origin", "https://other.example.com"),
        ]));
        assert!(!ok(&[
            ("host", "status.example.com"),
            ("origin", "https://status.example.com:8443"),
        ]));
        assert!(!ok(&[
            ("host", "127.0.0.1:3000"),
            ("origin", "https://status.example.com"),
        ]));
        assert!(!ok(&[("host", "status.example.com"), ("origin", "null")]));
        assert!(!ok(&[("origin", "https://status.example.com")]), "no host");
    }

    #[test]
    fn time_left_until_expiry() {
        let now = UNIX_EPOCH + Duration::from_secs(1_000);
        assert_eq!(time_left(&claims(1_600), now), Duration::from_secs(600));
        assert_eq!(time_left(&claims(1_000), now), Duration::ZERO);
        assert_eq!(time_left(&claims(10), now), Duration::ZERO);
    }
}
