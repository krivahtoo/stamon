//! Checks SSL certificate expiration.
//!
//! This crate will try to connect a remote server and check SSL certificate expiration.
//!
//! Basic usage example:
//!
//! ```rust
//! use ssl_exp::SslExpiration;
//!
//! let expiration = SslExpiration::from_domain_name("google.com").unwrap();
//! if expiration.is_expired() {
//!     // do something if SSL certificate expired
//! }
//! ```
//!
//! Based on https://github.com/VerKnowSys/ssl-expiration
//! by Onur Aslan and Daniel Dettlaff

use chrono::{DateTime, TimeDelta, Utc};
use error::{Error, Result};
use openssl::{
    asn1::*,
    ssl::{Ssl, SslContext, SslMethod, SslVerifyMode},
};
use std::{
    net::{TcpStream, ToSocketAddrs},
    ops::Add,
    os::raw::c_int,
    time::Duration,
};

pub mod error;

#[derive(Debug)]
pub struct SslExpiration(c_int, DateTime<Utc>);

impl SslExpiration {
    /// Creates new SslExpiration from domain name.
    ///
    /// This function will use HTTPS port (443) to check SSL certificate with 30 seconds timeout.
    pub fn from_domain_name(domain: &str) -> Result<SslExpiration> {
        SslExpiration::from_addr(format!("{}:443", domain), domain, 30) // seconds
    }

    /// This function will use HTTPS port (443) to check SSL certificate with custom timeout.
    pub fn from_domain_name_with_timeout(domain: &str, timeout: u64) -> Result<SslExpiration> {
        SslExpiration::from_addr(format!("{}:443", domain), domain, timeout)
    }

    /// Creates new SslExpiration from SocketAddr.
    pub fn from_addr<A: ToSocketAddrs>(
        addr: A,
        domain: &str,
        timeout: u64,
    ) -> Result<SslExpiration> {
        let context = {
            let mut context = SslContext::builder(SslMethod::tls())?;
            context.set_verify(SslVerifyMode::empty());
            context.build()
        };
        let mut connector = Ssl::new(&context)?;
        connector.set_hostname(domain)?;
        match addr.to_socket_addrs()?.next() {
            Some(first_address) => {
                let stream =
                    TcpStream::connect_timeout(&first_address, Duration::from_secs(timeout))?;
                stream.set_write_timeout(Some(Duration::from_secs(timeout)))?;
                stream.set_read_timeout(Some(Duration::from_secs(timeout)))?;

                let stream = connector
                    .connect(stream)
                    .map_err(|e| Error::Handshake(e.to_string()))?;
                let diff = Asn1Time::days_from_now(0)?.diff(
                    stream
                        .ssl()
                        .peer_certificate()
                        .ok_or(Error::NoCert)?
                        .not_after(),
                )?;

                Ok(SslExpiration(
                    diff.days * 24 * 60 * 60 + diff.secs,
                    Utc::now().add(TimeDelta::seconds(
                        (diff.days * 24 * 60 * 60 + diff.secs) as i64,
                    )),
                ))
            }
            None => Err(Error::Handshake(format!(
                "Couldn't resolve any address from domain: {}",
                &domain
            ))),
        }
    }

    /// How many seconds until SSL certificate expires.
    ///
    /// This function will return minus if SSL certificate is already expired.
    pub fn secs(&self) -> i32 {
        self.0
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.1
    }

    /// How many days until SSL certificate expires
    ///
    /// This function will return minus if SSL certificate is already expired.
    pub fn days(&self) -> i32 {
        self.0 / 60 / 60 / 24
    }

    /// Returns true if SSL certificate is expired
    pub fn is_expired(&self) -> bool {
        self.0 < 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssl_not_expired() {
        assert!(!SslExpiration::from_domain_name("google.com")
            .unwrap()
            .is_expired());
        let days = SslExpiration::from_domain_name("google.com")
            .unwrap()
            .days();
        assert!(days > 14)
    }

    #[test]
    fn test_too_small_timeout_chain() {
        SslExpiration::from_domain_name_with_timeout("google.com", 0).unwrap_err();
    }

    #[test]
    fn test_unresolvable_timeout_chain() {
        SslExpiration::from_domain_name_with_timeout("unresolvable.net", 3)
            .map_err(|e| println!("Error: {:?}", e))
            .unwrap_err();
    }

    #[test]
    fn test_sufficient_timeout_chain() {
        SslExpiration::from_domain_name_with_timeout("google.com", 30).unwrap();
    }

    #[test]
    fn test_non_panicing_chain() {
        SslExpiration::from_domain_name("google.com")
            .map(|validity| assert!(validity.days() > 14))
            .unwrap();
    }

    #[test]
    fn test_ssl_expired() {
        assert!(SslExpiration::from_domain_name("expired-rsa-dv.ssl.com")
            .unwrap()
            .is_expired());
    }
}
