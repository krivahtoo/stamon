use std::time::Instant;

use serde::{Deserialize, Serialize};
use sqlx::{
    ConnectOptions, Connection,
    mysql::{MySqlConnectOptions, MySqlSslMode},
    postgres::{PgConnectOptions, PgSslMode},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::CheckOutcome;

/// Shown instead of a stored password in API responses.
pub const REDACTED: &str = "********";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub engine: DatabaseEngine,
    pub host: String,
    /// Defaults to the engine's usual port.
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    /// Database name; for Redis, the database number.
    #[serde(default)]
    pub database: Option<String>,
    /// Require TLS. Without it, PostgreSQL and MySQL still use TLS when the
    /// server offers it.
    #[serde(default)]
    pub tls: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseEngine {
    Postgres,
    /// Also MariaDB.
    Mysql,
    Redis,
}

impl DatabaseEngine {
    fn default_port(self) -> u16 {
        match self {
            DatabaseEngine::Postgres => 5432,
            DatabaseEngine::Mysql => 3306,
            DatabaseEngine::Redis => 6379,
        }
    }
}

fn non_empty(value: &Option<String>) -> Option<&str> {
    value.as_deref().filter(|v| !v.is_empty())
}

impl DatabaseConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.host.trim().is_empty() {
            return Err("host must not be empty".into());
        }
        if self.port == Some(0) {
            return Err("port must be between 1 and 65535".into());
        }
        if self.engine == DatabaseEngine::Redis {
            if self.tls {
                return Err("TLS isn't supported for Redis checks yet".into());
            }
            if let Some(db) = non_empty(&self.database)
                && db.parse::<u32>().is_err()
            {
                return Err("database must be a number for Redis".into());
            }
        }
        Ok(())
    }

    fn port(&self) -> u16 {
        self.port.unwrap_or(self.engine.default_port())
    }

    /// Replace the password with a placeholder, for showing the config.
    pub fn redact(&mut self) {
        if self.password.is_some() {
            self.password = Some(REDACTED.into());
        }
    }

    /// Keep `previous`'s password when this one is still the placeholder,
    /// so a redacted config can be saved back unchanged.
    pub fn keep_password_from(&mut self, previous: &DatabaseConfig) {
        if self.password.as_deref() == Some(REDACTED) {
            self.password = previous.password.clone();
        }
    }

    async fn check_postgres(&self) -> Result<(), String> {
        let error = |e: sqlx::Error| e.to_string();
        let mut options = PgConnectOptions::new()
            .host(self.host.trim())
            .port(self.port())
            .ssl_mode(if self.tls {
                PgSslMode::Require
            } else {
                PgSslMode::Prefer
            });
        if let Some(username) = non_empty(&self.username) {
            options = options.username(username);
        }
        if let Some(password) = non_empty(&self.password) {
            options = options.password(password);
        }
        if let Some(database) = non_empty(&self.database) {
            options = options.database(database);
        }
        let mut conn = options.connect().await.map_err(error)?;
        sqlx::query("SELECT 1")
            .execute(&mut conn)
            .await
            .map_err(error)?;
        conn.close().await.map_err(error)
    }

    async fn check_mysql(&self) -> Result<(), String> {
        let error = |e: sqlx::Error| e.to_string();
        let mut options = MySqlConnectOptions::new()
            .host(self.host.trim())
            .port(self.port())
            .ssl_mode(if self.tls {
                MySqlSslMode::Required
            } else {
                MySqlSslMode::Preferred
            });
        if let Some(username) = non_empty(&self.username) {
            options = options.username(username);
        }
        if let Some(password) = non_empty(&self.password) {
            options = options.password(password);
        }
        if let Some(database) = non_empty(&self.database) {
            options = options.database(database);
        }
        let mut conn = options.connect().await.map_err(error)?;
        sqlx::query("SELECT 1")
            .execute(&mut conn)
            .await
            .map_err(error)?;
        conn.close().await.map_err(error)
    }

    async fn check_redis(&self) -> Result<(), String> {
        let stream = TcpStream::connect((self.host.trim(), self.port()))
            .await
            .map_err(|e| e.to_string())?;
        let mut conn = BufReader::new(stream);

        if let Some(password) = non_empty(&self.password) {
            match non_empty(&self.username) {
                Some(username) => redis_command(&mut conn, &["AUTH", username, password]).await?,
                None => redis_command(&mut conn, &["AUTH", password]).await?,
            };
        }
        if let Some(db) = non_empty(&self.database) {
            redis_command(&mut conn, &["SELECT", db]).await?;
        }
        match redis_command(&mut conn, &["PING"]).await?.as_str() {
            "PONG" => Ok(()),
            reply => Err(format!("Unexpected reply to PING: {reply}")),
        }
    }
}

/// Send a command and return its simple-string reply.
async fn redis_command(conn: &mut BufReader<TcpStream>, args: &[&str]) -> Result<String, String> {
    let mut command = format!("*{}\r\n", args.len());
    for arg in args {
        command.push_str(&format!("${}\r\n{arg}\r\n", arg.len()));
    }
    conn.get_mut()
        .write_all(command.as_bytes())
        .await
        .map_err(|e| e.to_string())?;

    let mut reply = String::new();
    conn.read_line(&mut reply)
        .await
        .map_err(|e| e.to_string())?;
    let reply = reply.trim_end();
    if let Some(ok) = reply.strip_prefix('+') {
        Ok(ok.to_owned())
    } else if let Some(err) = reply.strip_prefix('-') {
        Err(format!("Redis replied: {err}"))
    } else if reply.is_empty() {
        Err("Redis closed the connection".into())
    } else {
        Err(format!("Unexpected reply from Redis: {reply}"))
    }
}

/// Up when the database accepts a connection and answers a trivial query
/// (`SELECT 1`, or `PING` for Redis).
pub(crate) async fn check(config: &DatabaseConfig) -> CheckOutcome {
    let start = Instant::now();
    let result = match config.engine {
        DatabaseEngine::Postgres => config.check_postgres().await,
        DatabaseEngine::Mysql => config.check_mysql().await,
        DatabaseEngine::Redis => config.check_redis().await,
    };
    match result {
        Ok(()) => CheckOutcome::up(start.elapsed()),
        Err(e) => CheckOutcome::down(start.elapsed(), e),
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::*;
    use crate::CheckStatus;

    fn config(engine: DatabaseEngine, port: u16) -> DatabaseConfig {
        DatabaseConfig {
            engine,
            host: "127.0.0.1".into(),
            port: Some(port),
            username: None,
            password: None,
            database: None,
            tls: false,
        }
    }

    /// A Redis that wants `secret` and answers `PING`, returning the
    /// commands it received.
    async fn serve_redis() -> (u16, tokio::task::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut conn = BufReader::new(stream);
            let mut seen = Vec::new();
            loop {
                // Each command is `*N` then N pairs of `$len` and the argument.
                let mut header = String::new();
                if conn.read_line(&mut header).await.unwrap() == 0 {
                    break;
                }
                let count: usize = header.trim()[1..].parse().unwrap();
                let mut args = Vec::new();
                for _ in 0..count {
                    let mut len = String::new();
                    conn.read_line(&mut len).await.unwrap();
                    let mut arg = String::new();
                    conn.read_line(&mut arg).await.unwrap();
                    args.push(arg.trim_end().to_owned());
                }
                let reply = match args[0].as_str() {
                    "AUTH" if args.last().unwrap() == "secret" => "+OK\r\n",
                    "AUTH" => "-WRONGPASS invalid username-password pair\r\n",
                    "SELECT" => "+OK\r\n",
                    "PING" => "+PONG\r\n",
                    _ => "-ERR unknown command\r\n",
                };
                seen.push(args.join(" "));
                conn.get_mut().write_all(reply.as_bytes()).await.unwrap();
                if reply.starts_with('-') {
                    break;
                }
            }
            seen
        });
        (port, handle)
    }

    #[tokio::test]
    async fn redis_auth_select_and_ping() {
        let (port, server) = serve_redis().await;
        let redis = DatabaseConfig {
            username: Some("monitor".into()),
            password: Some("secret".into()),
            database: Some("2".into()),
            ..config(DatabaseEngine::Redis, port)
        };
        let outcome = check(&redis).await;
        assert_eq!(outcome.status, CheckStatus::Up, "{outcome:?}");
        assert_eq!(
            server.await.unwrap(),
            ["AUTH monitor secret", "SELECT 2", "PING"]
        );
    }

    #[tokio::test]
    async fn redis_wrong_password_is_down() {
        let (port, _server) = serve_redis().await;
        let redis = DatabaseConfig {
            password: Some("nope".into()),
            ..config(DatabaseEngine::Redis, port)
        };
        let outcome = check(&redis).await;
        assert_eq!(outcome.status, CheckStatus::Down);
        assert!(outcome.message.unwrap().contains("WRONGPASS"));
    }

    #[tokio::test]
    async fn unreachable_databases_are_down() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        for engine in [
            DatabaseEngine::Postgres,
            DatabaseEngine::Mysql,
            DatabaseEngine::Redis,
        ] {
            let outcome = check(&config(engine, port)).await;
            assert_eq!(outcome.status, CheckStatus::Down, "{engine:?}: {outcome:?}");
        }
    }

    #[test]
    fn redaction_round_trip() {
        let stored = DatabaseConfig {
            password: Some("hunter2".into()),
            ..config(DatabaseEngine::Postgres, 5432)
        };
        let mut shown = stored.clone();
        shown.redact();
        assert_eq!(shown.password.as_deref(), Some(REDACTED));

        // Saving the redacted config back keeps the real password.
        let mut saved = shown.clone();
        saved.keep_password_from(&stored);
        assert_eq!(saved, stored);

        // A new password replaces it.
        let mut changed = DatabaseConfig {
            password: Some("new".into()),
            ..shown
        };
        changed.keep_password_from(&stored);
        assert_eq!(changed.password.as_deref(), Some("new"));

        let mut none = config(DatabaseEngine::Postgres, 5432);
        none.redact();
        assert_eq!(none.password, None, "no password, nothing to hide");
    }

    #[test]
    fn validation() {
        assert!(config(DatabaseEngine::Postgres, 5432).validate().is_ok());
        assert!(
            DatabaseConfig {
                host: " ".into(),
                ..config(DatabaseEngine::Mysql, 3306)
            }
            .validate()
            .is_err()
        );
        assert!(
            DatabaseConfig {
                tls: true,
                ..config(DatabaseEngine::Redis, 6379)
            }
            .validate()
            .is_err()
        );
        assert!(
            DatabaseConfig {
                database: Some("cache".into()),
                ..config(DatabaseEngine::Redis, 6379)
            }
            .validate()
            .is_err()
        );
        assert_eq!(
            DatabaseConfig {
                port: None,
                ..config(DatabaseEngine::Mysql, 1)
            }
            .port(),
            3306
        );
    }
}
