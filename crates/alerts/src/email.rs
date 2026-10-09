use std::time::Duration;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use serde::{Deserialize, Serialize};

use crate::Alert;

fn default_port() -> u16 {
    587
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmailConfig {
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub security: EmailSecurity,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    /// Sender, e.g. `Stamon <stamon@example.com>`.
    pub from: String,
    pub to: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailSecurity {
    /// Upgrade a plain connection with STARTTLS (usually port 587).
    #[default]
    Starttls,
    /// TLS from the start (usually port 465).
    Tls,
    /// No encryption; only for local relays.
    None,
}

fn parse_mailbox(field: &str, address: &str) -> Result<Mailbox, String> {
    address
        .parse()
        .map_err(|e| format!("invalid {field} address {address}: {e}"))
}

impl EmailConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("host must not be empty".into());
        }
        parse_mailbox("from", &self.from)?;
        if self.to.is_empty() {
            return Err("to needs at least one address".into());
        }
        for address in &self.to {
            parse_mailbox("to", address)?;
        }
        Ok(())
    }

    fn message(&self, alert: &Alert) -> Result<Message, String> {
        let mut builder = Message::builder()
            .from(parse_mailbox("from", &self.from)?)
            .subject(format!("[Stamon] {}", alert.title()))
            .header(ContentType::TEXT_PLAIN);
        for address in &self.to {
            builder = builder.to(parse_mailbox("to", address)?);
        }
        builder
            .body(format!("{}\n\n{}\n", alert.title(), alert.details()))
            .map_err(|e| format!("Failed to build email: {e}"))
    }

    fn transport(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>, String> {
        let host = self.host.trim();
        let builder = match self.security {
            EmailSecurity::Starttls => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host),
            EmailSecurity::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(host),
            EmailSecurity::None => Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                host,
            )),
        }
        .map_err(|e| format!("Failed to set up SMTP connection: {e}"))?;

        let mut builder = builder
            .port(self.port)
            .timeout(Some(Duration::from_secs(10)));
        if let Some(username) = self.username.as_deref().filter(|u| !u.is_empty()) {
            builder = builder.credentials(Credentials::new(
                username.to_owned(),
                self.password.clone().unwrap_or_default(),
            ));
        }
        Ok(builder.build())
    }
}

pub(crate) async fn send(config: &EmailConfig, alert: &Alert) -> Result<(), String> {
    let message = config.message(alert)?;
    config
        .transport()?
        .send(message)
        .await
        .map_err(|e| format!("Failed to send email: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::TcpListener,
        sync::oneshot,
    };

    use super::*;
    use crate::{AlertStatus, tests::alert};

    /// Speak just enough SMTP to accept one message and return its data.
    async fn serve() -> (u16, oneshot::Receiver<(Vec<String>, String)>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (read, mut write) = stream.into_split();
            let mut lines = BufReader::new(read).lines();
            write.write_all(b"220 test ESMTP\r\n").await.unwrap();

            let mut recipients = Vec::new();
            let mut data = String::new();
            let mut in_data = false;
            while let Some(line) = lines.next_line().await.unwrap() {
                if in_data {
                    if line == "." {
                        in_data = false;
                        write.write_all(b"250 queued\r\n").await.unwrap();
                    } else {
                        data.push_str(&line);
                        data.push('\n');
                    }
                    continue;
                }
                let command = line.to_ascii_uppercase();
                let reply: &[u8] = if command.starts_with("EHLO") {
                    b"250 test\r\n"
                } else if command.starts_with("RCPT TO:") {
                    recipients.push(line[8..].to_owned());
                    b"250 ok\r\n"
                } else if command.starts_with("DATA") {
                    in_data = true;
                    b"354 go ahead\r\n"
                } else if command.starts_with("QUIT") {
                    write.write_all(b"221 bye\r\n").await.unwrap();
                    break;
                } else {
                    b"250 ok\r\n"
                };
                write.write_all(reply).await.unwrap();
            }
            let _ = tx.send((recipients, data));
        });
        (port, rx)
    }

    fn config(port: u16) -> EmailConfig {
        EmailConfig {
            host: "127.0.0.1".into(),
            port,
            security: EmailSecurity::None,
            username: None,
            password: None,
            from: "Stamon <stamon@example.com>".into(),
            to: vec!["ops@example.com".into(), "Dev <dev@example.com>".into()],
        }
    }

    #[tokio::test]
    async fn sends_plain_text_email_to_every_recipient() {
        let (port, received) = serve().await;
        send(&config(port), &alert(AlertStatus::Down))
            .await
            .unwrap();

        let (recipients, data) = received.await.unwrap();
        assert_eq!(recipients, ["<ops@example.com>", "<dev@example.com>"]);
        assert!(data.contains("Subject: [Stamon] API is down"), "{data}");
        assert!(data.contains("Unexpected status code: 500"), "{data}");
    }

    #[tokio::test]
    async fn unreachable_server_is_an_error() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        let err = send(&config(port), &alert(AlertStatus::Down))
            .await
            .unwrap_err();
        assert!(err.starts_with("Failed to send email"), "{err}");
    }

    #[test]
    fn validation() {
        assert!(config(25).validate().is_ok());
        let mut bad = config(25);
        bad.to.clear();
        assert!(bad.validate().is_err());
        bad.to = vec!["not an address".into()];
        assert!(bad.validate().is_err());
        let mut bad_from = config(25);
        bad_from.from = "nobody".into();
        assert!(bad_from.validate().is_err());

        let defaults: EmailConfig = serde_json::from_value(serde_json::json!({
            "host": "smtp.example.com",
            "from": "a@example.com",
            "to": ["b@example.com"],
        }))
        .unwrap();
        assert_eq!(defaults.port, 587);
        assert_eq!(defaults.security, EmailSecurity::Starttls);
    }
}
