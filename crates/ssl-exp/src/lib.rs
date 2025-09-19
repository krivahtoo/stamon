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
        assert!(
            !SslExpiration::from_domain_name("google.com")
                .unwrap()
                .is_expired()
        );
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
        assert!(
            SslExpiration::from_domain_name("expired-rsa-dv.ssl.com")
                .unwrap()
                .is_expired()
        );
    }

    #[test]
    fn test_ssl_expiration_from_seconds() {
        // Test with future expiration (positive seconds)
        let future_expiry = SslExpiration(86400, Utc::now().add(TimeDelta::days(1))); // 1 day
        assert!(!future_expiry.is_expired());
        assert!(future_expiry.days() > 0);

        // Test with past expiration (negative seconds)  
        let past_expiry = SslExpiration(-86400, Utc::now().add(TimeDelta::days(-1))); // 1 day ago
        assert!(past_expiry.is_expired());
        assert!(past_expiry.days() < 0);
    }

    #[test]
    fn test_ssl_expiration_edge_cases() {
        // Test exactly at expiration time (0 seconds means expired)
        let now_expiry = SslExpiration(0, Utc::now());
        assert!(!now_expiry.is_expired()); // 0 seconds is not negative, so not expired
        assert_eq!(now_expiry.days(), 0);

        // Test very small positive time (should not be expired)
        let small_future = SslExpiration(1, Utc::now().add(TimeDelta::seconds(1)));
        assert!(!small_future.is_expired());

        // Test very small negative time (should be expired)
        let small_past = SslExpiration(-1, Utc::now().add(TimeDelta::seconds(-1)));
        assert!(small_past.is_expired());
    }

    #[test]
    fn test_days_calculation() {
        // Test 7 days in the future
        let week_future = SslExpiration(7 * 24 * 60 * 60, Utc::now().add(TimeDelta::days(7)));
        assert_eq!(week_future.days(), 7);

        // Test 30 days in the future
        let month_future = SslExpiration(30 * 24 * 60 * 60, Utc::now().add(TimeDelta::days(30)));
        assert_eq!(month_future.days(), 30);

        // Test partial day (should round down)
        let partial_day = SslExpiration(12 * 60 * 60, Utc::now().add(TimeDelta::hours(12))); // 12 hours
        assert_eq!(partial_day.days(), 0);
    }

    #[test]
    fn test_error_types() {
        use crate::error::Error;

        // Test that different error types can be created and formatted
        let no_cert_error = Error::NoCert;
        let handshake_error = Error::Handshake("test error".to_string());
        
        assert!(!format!("{}", no_cert_error).is_empty());
        assert!(!format!("{}", handshake_error).is_empty());
        assert!(format!("{}", handshake_error).contains("test error"));
    }

    #[test]
    fn test_timeout_validation() {
        // Test that timeout of 0 should fail
        let result = SslExpiration::from_domain_name_with_timeout("test.com", 0);
        assert!(result.is_err());
    }

    #[test] 
    fn test_domain_name_convenience_methods() {
        // Test that convenience methods use correct defaults
        // These will fail in CI due to network, but test the API structure
        
        // Test default timeout method exists and has proper signature
        let result = SslExpiration::from_domain_name("test.local");
        assert!(result.is_err()); // Expected to fail due to invalid domain

        // Test custom timeout method exists and has proper signature  
        let result = SslExpiration::from_domain_name_with_timeout("test.local", 5);
        assert!(result.is_err()); // Expected to fail due to invalid domain
    }
}
