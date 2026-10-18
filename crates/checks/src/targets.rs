//! Keeping checks to public addresses, for instances anyone can use (like
//! the public demo), so monitors can't be used to probe the host's network.
//!
//! Addresses are checked after resolving and the check then connects to the
//! checked address, so a name can't resolve to a public address when
//! validated and a private one when connecting.

use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::{Arc, OnceLock},
    time::Duration,
};

use reqwest::{
    Client,
    dns::{Addrs, Name, Resolve, Resolving},
    redirect::{Attempt, Policy},
};

use crate::{CheckConfig, CheckOutcome, PingConfig, TcpConfig, dns, http, ping, tcp, tls};

/// Whether `ip` is reachable on the public internet, rather than loopback,
/// a private or link-local network, or another special-purpose range.
pub fn is_public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => is_public_v4(ip),
        IpAddr::V6(ip) => match ip.to_ipv4_mapped() {
            Some(ip) => is_public_v4(ip),
            None => is_public_v6(ip),
        },
    }
}

fn is_public_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    !(ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_multicast()
        || a == 0
        // Shared address space (carrier-grade NAT)
        || (a == 100 && (64..128).contains(&b))
        // IETF protocol assignments
        || (a == 192 && b == 0 && c == 0)
        // Benchmarking
        || (a == 198 && (b == 18 || b == 19))
        // Reserved
        || a >= 240)
}

fn is_public_v6(ip: Ipv6Addr) -> bool {
    let segments = ip.segments();
    // NAT64 addresses reach the IPv4 address they embed.
    if segments[..6] == [0x64, 0xff9b, 0, 0, 0, 0] {
        let [.., a, b, c, d] = ip.octets();
        return is_public_v4(Ipv4Addr::new(a, b, c, d));
    }
    !(ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_multicast()
        // Unique local
        || (segments[0] & 0xfe00) == 0xfc00
        // Link local
        || (segments[0] & 0xffc0) == 0xfe80
        // Documentation
        || (segments[0] == 0x2001 && segments[1] == 0x0db8))
}

fn refuse(host: &str, ip: IpAddr) -> String {
    if literal_ip(host) == Some(ip) {
        format!("{ip} isn't a public address")
    } else {
        format!("{host} is at {ip}, which isn't a public address")
    }
}

/// The IP address in `host`, if it is one, with IPv6 brackets removed.
fn literal_ip(host: &str) -> Option<IpAddr> {
    host.trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .parse()
        .ok()
}

fn is_localhost(host: &str) -> bool {
    let host = host.trim().trim_end_matches('.').to_ascii_lowercase();
    host == "localhost" || host.ends_with(".localhost")
}

/// Reject a host that is plainly not public, without resolving it.
fn validate_host(host: &str) -> Result<(), String> {
    if is_localhost(host) {
        return Err(format!("{host} isn't a public address"));
    }
    match literal_ip(host) {
        Some(ip) if !is_public(ip) => Err(refuse(host, ip)),
        _ => Ok(()),
    }
}

/// The host part of an http(s) URL.
fn url_host(url: &str) -> Result<String, String> {
    reqwest::Url::parse(url)
        .map_err(|e| format!("invalid url: {e}"))?
        .host_str()
        .map(str::to_owned)
        .ok_or_else(|| "url must have a host".into())
}

/// Resolve `host` and check every address it has is public.
async fn resolve_public(host: &str, port: u16) -> Result<Vec<SocketAddr>, CheckOutcome> {
    let host = host.trim().trim_start_matches('[').trim_end_matches(']');
    let addrs: Vec<SocketAddr> = match tokio::net::lookup_host((host, port)).await {
        Ok(addrs) => addrs.collect(),
        Err(e) => {
            return Err(CheckOutcome::down(
                Duration::ZERO,
                format!("Failed to resolve {host}: {e}"),
            ));
        }
    };
    if addrs.is_empty() {
        return Err(CheckOutcome::down(
            Duration::ZERO,
            format!("No address found for {host}"),
        ));
    }
    if let Some(addr) = addrs.iter().find(|addr| !is_public(addr.ip())) {
        return Err(CheckOutcome::error(refuse(host, addr.ip())));
    }
    Ok(addrs)
}

/// Resolves names for the HTTP client, refusing names with a non-public
/// address. Covers redirects too, since every request resolves through it.
struct PublicResolver;

impl Resolve for PublicResolver {
    fn resolve(&self, name: Name) -> Resolving {
        Box::pin(async move {
            let addrs = resolve_public(name.as_str(), 0)
                .await
                .map_err(|outcome| outcome.message.unwrap_or_default())?;
            Ok(Box::new(addrs.into_iter()) as Addrs)
        })
    }
}

/// Follow redirects like the default policy, except to a non-public IP
/// address, which skips the resolver.
fn follow_public(attempt: Attempt) -> reqwest::redirect::Action {
    if attempt.previous().len() >= 10 {
        return attempt.error("too many redirects");
    }
    let refused = attempt.url().host_str().and_then(|host| {
        literal_ip(host)
            .filter(|ip| !is_public(*ip))
            .map(|ip| refuse(host, ip))
    });
    match refused {
        Some(message) => attempt.error(message),
        None => attempt.follow(),
    }
}

fn public_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        Client::builder()
            .dns_resolver(Arc::new(PublicResolver))
            .redirect(Policy::custom(follow_public))
            // A proxy would connect to whatever it is asked to.
            .no_proxy()
            .build()
            .expect("HTTP client should build")
    })
}

const UNAVAILABLE: &str = "Docker and database checks are turned off on this instance";

impl CheckConfig {
    /// Reject configs that can't be checked when checks are kept to public
    /// addresses. Hostnames are only checked when the check runs.
    pub fn validate_public(&self) -> Result<(), String> {
        match self {
            CheckConfig::Http(config) => validate_host(&url_host(&config.url)?),
            CheckConfig::Ping(config) => validate_host(&ping::host(&config.host)),
            CheckConfig::Tcp(config) => validate_host(&config.host),
            CheckConfig::Tls(config) => validate_host(&config.host),
            CheckConfig::Dns(config) => match dns::resolver_addr(config) {
                Some(addr) if !is_public(addr.ip()) => {
                    Err(format!("resolver {} isn't a public address", addr.ip()))
                }
                _ => Ok(()),
            },
            CheckConfig::Docker(_) | CheckConfig::Database(_) => Err(UNAVAILABLE.into()),
            CheckConfig::Push(_) => Ok(()),
        }
    }
}

/// Run the check, connecting only to public addresses.
pub(crate) async fn check(config: &CheckConfig, timeout: Duration) -> CheckOutcome {
    if let Err(message) = config.validate_public() {
        return CheckOutcome::error(message);
    }
    match config {
        CheckConfig::Http(config) => http::check_with(config, public_client()).await,
        CheckConfig::Ping(config) => match resolve_public(&ping::host(&config.host), 0).await {
            Ok(addrs) => {
                let host = addrs[0].ip().to_string();
                ping::check(&PingConfig { host }, timeout).await
            }
            Err(outcome) => outcome,
        },
        CheckConfig::Tcp(config) => match resolve_public(&config.host, config.port).await {
            Ok(addrs) => {
                let config = TcpConfig {
                    host: addrs[0].ip().to_string(),
                    port: config.port,
                };
                tcp::check(&config).await
            }
            Err(outcome) => outcome,
        },
        CheckConfig::Tls(config) => match resolve_public(&config.host, config.port).await {
            Ok(addrs) => tls::check_addrs(config, addrs, timeout).await,
            Err(outcome) => outcome,
        },
        // Only asks a public resolver (or the system's) about a name.
        CheckConfig::Dns(config) => dns::check(config).await,
        CheckConfig::Docker(_) | CheckConfig::Database(_) => CheckOutcome::error(UNAVAILABLE),
        CheckConfig::Push(_) => config.run(timeout).await,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{CheckStatus, HttpConfig, Targets};

    #[test]
    fn public_addresses() {
        for ip in [
            "1.1.1.1",
            "93.184.216.34",
            "2606:4700::1111",
            "::ffff:8.8.8.8",
        ] {
            assert!(is_public(ip.parse().unwrap()), "{ip}");
        }
        for ip in [
            "127.0.0.1",
            "10.1.2.3",
            "172.16.0.1",
            "192.168.1.1",
            "169.254.169.254",
            "100.64.0.1",
            "0.0.0.0",
            "255.255.255.255",
            "::1",
            "::",
            "fd00::1",
            "fe80::1",
            "::ffff:127.0.0.1",
            "64:ff9b::a00:1",
        ] {
            assert!(!is_public(ip.parse().unwrap()), "{ip}");
        }
    }

    #[test]
    fn validation_rejects_plainly_private_targets() {
        let config = |value| serde_json::from_value::<CheckConfig>(value).unwrap();
        for value in [
            json!({ "type": "http", "url": "http://169.254.169.254/latest/meta-data" }),
            json!({ "type": "http", "url": "http://localhost:3000/" }),
            json!({ "type": "http", "url": "http://[::1]/" }),
            json!({ "type": "ping", "host": "10.0.0.1" }),
            json!({ "type": "tcp", "host": "127.0.0.1", "port": 22 }),
            json!({ "type": "tls", "host": "app.localhost" }),
            json!({ "type": "dns", "host": "example.com", "resolver": "192.168.1.1" }),
            json!({ "type": "docker", "container": "web" }),
            json!({ "type": "database", "engine": "redis", "host": "redis.example.com" }),
        ] {
            assert!(config(value.clone()).validate_public().is_err(), "{value}");
        }
        for value in [
            json!({ "type": "http", "url": "https://example.com/" }),
            json!({ "type": "ping", "host": "1.1.1.1" }),
            json!({ "type": "tcp", "host": "example.com", "port": 443 }),
            json!({ "type": "dns", "host": "example.com", "resolver": "1.1.1.1" }),
            json!({ "type": "push", "token": "abcdefgh12345678" }),
        ] {
            assert!(config(value.clone()).validate_public().is_ok(), "{value}");
        }
    }

    async fn serve_redirect(to: &'static str) -> String {
        use axum::{Router, response::Redirect, routing::get};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let app = Router::new().route("/", get(move || async move { Redirect::to(to) }));
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        format!("http://{addr}/")
    }

    #[tokio::test]
    async fn names_resolving_to_private_addresses_are_refused() {
        // Not an IP address to Rust, but resolves to 127.0.0.1.
        let config = CheckConfig::Tcp(TcpConfig {
            host: "127.1".into(),
            port: 80,
        });
        assert!(config.validate_public().is_ok());
        let outcome = config
            .check_targets(Duration::from_secs(2), Targets::Public)
            .await;
        assert_eq!(outcome.status, CheckStatus::Error);
        assert!(outcome.message.unwrap().contains("public"));
    }

    #[tokio::test]
    async fn redirects_to_private_addresses_are_refused() {
        // The server itself is on localhost, so only the redirect policy is
        // tested here, through the public client.
        let url = serve_redirect("http://127.0.0.1:1/").await;
        let outcome = http::check_with(&HttpConfig::get(&url), public_client()).await;
        assert_eq!(outcome.status, CheckStatus::Down);
        assert!(outcome.message.unwrap().contains("redirect"));
    }
}
