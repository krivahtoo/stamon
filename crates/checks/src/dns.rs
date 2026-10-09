use std::{
    net::{IpAddr, SocketAddr},
    time::Instant,
};

use hickory_resolver::{
    Resolver, TokioResolver,
    config::{NameServerConfigGroup, ResolverConfig},
    name_server::TokioConnectionProvider,
    proto::rr::{RData, RecordType},
};
use serde::{Deserialize, Serialize};

use crate::CheckOutcome;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsConfig {
    pub host: String,
    #[serde(default)]
    pub record_type: DnsRecordType,
    /// Nameserver to ask, as `ip` or `ip:port`. Defaults to the system resolver.
    #[serde(default)]
    pub resolver: Option<String>,
    /// Values that must all be among the answers, e.g. `93.184.215.14` or
    /// `10 mail.example.com`. Without any, an answer of any value is up.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expected: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DnsRecordType {
    #[default]
    A,
    Aaaa,
    Cname,
    Mx,
    Ns,
    Txt,
}

impl From<DnsRecordType> for RecordType {
    fn from(record_type: DnsRecordType) -> Self {
        match record_type {
            DnsRecordType::A => RecordType::A,
            DnsRecordType::Aaaa => RecordType::AAAA,
            DnsRecordType::Cname => RecordType::CNAME,
            DnsRecordType::Mx => RecordType::MX,
            DnsRecordType::Ns => RecordType::NS,
            DnsRecordType::Txt => RecordType::TXT,
        }
    }
}

fn parse_resolver(resolver: &str) -> Result<SocketAddr, String> {
    let resolver = resolver.trim();
    resolver
        .parse::<SocketAddr>()
        .or_else(|_| resolver.parse::<IpAddr>().map(|ip| SocketAddr::new(ip, 53)))
        .map_err(|_| format!("resolver must be an IP address or ip:port, got {resolver}"))
}

/// Compare answers ignoring case and the trailing dot of names.
fn normalize(value: &str) -> String {
    value.trim().trim_end_matches('.').to_lowercase()
}

fn answer_text(data: &RData) -> String {
    match data {
        RData::TXT(txt) => txt
            .iter()
            .map(|part| String::from_utf8_lossy(part))
            .collect(),
        other => other.to_string(),
    }
}

impl DnsConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("host must not be empty".into());
        }
        if let Some(resolver) = self.resolver.as_deref().filter(|r| !r.trim().is_empty()) {
            parse_resolver(resolver)?;
        }
        Ok(())
    }

    fn resolver(&self) -> Result<TokioResolver, String> {
        let mut builder = match self.resolver.as_deref().filter(|r| !r.trim().is_empty()) {
            Some(resolver) => {
                let addr = parse_resolver(resolver)?;
                let servers =
                    NameServerConfigGroup::from_ips_clear(&[addr.ip()], addr.port(), true);
                Resolver::builder_with_config(
                    ResolverConfig::from_parts(None, vec![], servers),
                    TokioConnectionProvider::default(),
                )
            }
            None => Resolver::builder_tokio()
                .map_err(|e| format!("Failed to load the system DNS configuration: {e}"))?,
        };
        // Every check should ask the nameserver rather than reuse a cached answer.
        builder.options_mut().cache_size = 0;
        Ok(builder.build())
    }
}

pub(crate) async fn check(config: &DnsConfig) -> CheckOutcome {
    let start = Instant::now();
    let resolver = match config.resolver() {
        Ok(resolver) => resolver,
        Err(e) => return CheckOutcome::error(e),
    };
    let host = config.host.trim();
    let record_type = RecordType::from(config.record_type);

    let lookup = match resolver.lookup(host, record_type).await {
        Ok(lookup) => lookup,
        Err(e) => {
            return CheckOutcome::down(
                start.elapsed(),
                format!("{record_type} lookup for {host} failed: {e}"),
            );
        }
    };
    let answers: Vec<String> = lookup.iter().map(answer_text).collect();
    if answers.is_empty() {
        return CheckOutcome::down(
            start.elapsed(),
            format!("No {record_type} records found for {host}"),
        );
    }

    let found: Vec<String> = answers.iter().map(|a| normalize(a)).collect();
    let missing: Vec<&str> = config
        .expected
        .iter()
        .filter(|expected| !found.contains(&normalize(expected)))
        .map(String::as_str)
        .collect();
    if !missing.is_empty() {
        return CheckOutcome::down(
            start.elapsed(),
            format!(
                "Expected {} in {record_type} records for {host}, got {}",
                missing.join(", "),
                answers.join(", ")
            ),
        );
    }
    CheckOutcome::up(start.elapsed())
}

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, str::FromStr};

    use hickory_resolver::proto::{
        op::{Message, MessageType, ResponseCode},
        rr::{Name, Record, rdata},
    };
    use tokio::net::UdpSocket;

    use super::*;
    use crate::CheckStatus;

    /// Answers A queries for `app.test.` with two addresses and MX queries
    /// for `test.` with one exchange; everything else gets NXDOMAIN.
    async fn serve() -> String {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let addr = socket.local_addr().unwrap();
        tokio::spawn(async move {
            let mut buf = [0u8; 512];
            loop {
                let (len, peer) = socket.recv_from(&mut buf).await.unwrap();
                let request = Message::from_vec(&buf[..len]).unwrap();
                let query = request.queries()[0].clone();
                let mut response = Message::new();
                response
                    .set_id(request.id())
                    .set_message_type(MessageType::Response)
                    .set_recursion_desired(true)
                    .set_recursion_available(true)
                    .add_query(query.clone());

                let name = query.name().clone();
                match (name.to_ascii().as_str(), query.query_type()) {
                    ("app.test.", RecordType::A) => {
                        for ip in [[10, 0, 0, 1], [10, 0, 0, 2]] {
                            response.add_answer(Record::from_rdata(
                                name.clone(),
                                60,
                                RData::A(rdata::A(Ipv4Addr::from(ip))),
                            ));
                        }
                    }
                    ("test.", RecordType::MX) => {
                        response.add_answer(Record::from_rdata(
                            name.clone(),
                            60,
                            RData::MX(rdata::MX::new(10, Name::from_str("mail.test.").unwrap())),
                        ));
                    }
                    _ => {
                        response.set_response_code(ResponseCode::NXDomain);
                    }
                }
                socket
                    .send_to(&response.to_vec().unwrap(), peer)
                    .await
                    .unwrap();
            }
        });
        addr.to_string()
    }

    fn config(host: &str, record_type: DnsRecordType, resolver: &str) -> DnsConfig {
        DnsConfig {
            host: host.into(),
            record_type,
            resolver: Some(resolver.into()),
            expected: vec![],
        }
    }

    #[tokio::test]
    async fn answers_are_up() {
        let resolver = serve().await;
        let outcome = check(&config("app.test.", DnsRecordType::A, &resolver)).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
    }

    #[tokio::test]
    async fn missing_name_is_down() {
        let resolver = serve().await;
        let outcome = check(&config("gone.test.", DnsRecordType::A, &resolver)).await;
        assert_eq!(outcome.status, CheckStatus::Down, "{outcome:?}");
    }

    #[tokio::test]
    async fn expected_values_must_all_be_answered() {
        let resolver = serve().await;
        let mut a = config("app.test.", DnsRecordType::A, &resolver);
        a.expected = vec!["10.0.0.2".into()];
        assert_eq!(check(&a).await.status, CheckStatus::Up);

        a.expected = vec!["10.0.0.2".into(), "10.0.0.9".into()];
        let outcome = check(&a).await;
        assert_eq!(outcome.status, CheckStatus::Down);
        assert!(outcome.message.unwrap().contains("10.0.0.9"));

        // Names compare without case or trailing dot.
        let mut mx = config("test.", DnsRecordType::Mx, &resolver);
        mx.expected = vec!["10 MAIL.test".into()];
        let outcome = check(&mx).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
    }

    #[test]
    fn validation() {
        let mut dns = config("example.com", DnsRecordType::A, "1.1.1.1");
        assert!(dns.validate().is_ok());
        dns.resolver = Some("127.0.0.1:5353".into());
        assert!(dns.validate().is_ok());
        dns.resolver = Some("dns.google".into());
        assert!(dns.validate().is_err());
        dns.resolver = None;
        assert!(dns.validate().is_ok());
        dns.host = " ".into();
        assert!(dns.validate().is_err());
    }
}
