use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use ssl_exp::SslExpiration;

use crate::CheckOutcome;

fn default_port() -> u16 {
    443
}

fn default_warn_days() -> u16 {
    14
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TlsConfig {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    /// Report down when the certificate expires within this many days.
    #[serde(default = "default_warn_days")]
    pub warn_days: u16,
}

impl TlsConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("host must not be empty".into());
        }
        if self.port == 0 {
            return Err("port must be between 1 and 65535".into());
        }
        Ok(())
    }
}

/// Up while the server's certificate is more than `warn_days` from expiring.
pub(crate) async fn check(config: &TlsConfig, timeout: Duration) -> CheckOutcome {
    let start = Instant::now();
    let host = config.host.trim().to_owned();
    let port = config.port;
    let timeout_secs = timeout.as_secs().max(1);

    let addrs: Vec<SocketAddr> = match tokio::net::lookup_host((host.as_str(), port)).await {
        Ok(addrs) => addrs.collect(),
        Err(e) => {
            return CheckOutcome::down(start.elapsed(), format!("Failed to resolve {host}: {e}"));
        }
    };

    // The certificate lookup uses blocking sockets. Like a TCP connect, try
    // each address until one answers.
    let expiration = tokio::task::spawn_blocking(move || {
        let mut last_error = None;
        for addr in addrs {
            match SslExpiration::from_addr(addr, &host, timeout_secs) {
                Ok(expiration) => return Ok(expiration),
                Err(e) => last_error = Some(e.to_string()),
            }
        }
        Err(last_error.unwrap_or_else(|| format!("No address found for {host}")))
    })
    .await;

    let host = config.host.trim();
    let expiration = match expiration {
        Ok(Ok(expiration)) => expiration,
        Ok(Err(e)) => {
            return CheckOutcome::down(
                start.elapsed(),
                format!("TLS connection to {host}:{port} failed: {e}"),
            );
        }
        Err(e) => return CheckOutcome::error(format!("TLS check did not finish: {e}")),
    };

    let expires = expiration.date().format("%Y-%m-%d");
    if expiration.is_expired() {
        CheckOutcome::down(start.elapsed(), format!("Certificate expired on {expires}"))
    } else if expiration.days() < i32::from(config.warn_days) {
        CheckOutcome::down(
            start.elapsed(),
            format!(
                "Certificate expires in {} days, on {expires}",
                expiration.days()
            ),
        )
    } else {
        CheckOutcome::up(start.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::TcpListener,
        time::{SystemTime, UNIX_EPOCH},
    };

    use openssl::{
        asn1::Asn1Time,
        bn::BigNum,
        ec::{EcGroup, EcKey},
        hash::MessageDigest,
        nid::Nid,
        pkey::PKey,
        ssl::{SslAcceptor, SslMethod},
        x509::{X509, X509NameBuilder},
    };

    use super::*;
    use crate::CheckStatus;

    /// Serve TLS on localhost with a self-signed certificate that expires
    /// `days_valid` days from now (negative for an expired one).
    fn serve(days_valid: i64) -> u16 {
        let key = PKey::from_ec_key(
            EcKey::generate(&EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap()).unwrap(),
        )
        .unwrap();
        let mut name = X509NameBuilder::new().unwrap();
        name.append_entry_by_text("CN", "localhost").unwrap();
        let name = name.build();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let mut cert = X509::builder().unwrap();
        cert.set_version(2).unwrap();
        cert.set_serial_number(&BigNum::from_u32(1).unwrap().to_asn1_integer().unwrap())
            .unwrap();
        cert.set_subject_name(&name).unwrap();
        cert.set_issuer_name(&name).unwrap();
        cert.set_pubkey(&key).unwrap();
        cert.set_not_before(&Asn1Time::from_unix(now - 10 * 86_400).unwrap())
            .unwrap();
        cert.set_not_after(&Asn1Time::from_unix(now + days_valid * 86_400).unwrap())
            .unwrap();
        cert.sign(&key, MessageDigest::sha256()).unwrap();
        let cert = cert.build();

        let mut acceptor = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
        acceptor.set_private_key(&key).unwrap();
        acceptor.set_certificate(&cert).unwrap();
        let acceptor = acceptor.build();

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let _ = acceptor.accept(stream);
            }
        });
        port
    }

    fn config(port: u16, warn_days: u16) -> TlsConfig {
        TlsConfig {
            host: "localhost".into(),
            port,
            warn_days,
        }
    }

    #[tokio::test]
    async fn valid_certificate_is_up() {
        let port = serve(30);
        let outcome = check(&config(port, 14), Duration::from_secs(5)).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
    }

    #[tokio::test]
    async fn certificate_inside_warning_window_is_down() {
        let port = serve(30);
        let outcome = check(&config(port, 60), Duration::from_secs(5)).await;
        assert_eq!(outcome.status, CheckStatus::Down, "{outcome:?}");
        let message = outcome.message.unwrap();
        assert!(message.starts_with("Certificate expires in "), "{message}");
    }

    #[tokio::test]
    async fn expired_certificate_is_down() {
        let port = serve(-1);
        let outcome = check(&config(port, 14), Duration::from_secs(5)).await;
        assert_eq!(outcome.status, CheckStatus::Down, "{outcome:?}");
        assert!(outcome.message.unwrap().contains("expired on"));
    }

    #[tokio::test]
    async fn no_tls_server_is_down() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        let outcome = check(&config(port, 14), Duration::from_secs(5)).await;
        assert_eq!(outcome.status, CheckStatus::Down, "{outcome:?}");
    }

    #[test]
    fn defaults_and_validation() {
        let config: TlsConfig =
            serde_json::from_value(serde_json::json!({ "host": "example.com" })).unwrap();
        assert_eq!(config.port, 443);
        assert_eq!(config.warn_days, 14);
        assert!(config.validate().is_ok());
        assert!(
            TlsConfig {
                port: 0,
                ..config.clone()
            }
            .validate()
            .is_err()
        );
    }
}
