//! Sample data for the public demo, so visitors see a working dashboard.

use alerts::{ChannelConfig, WebhookConfig};
use checks::CheckConfig;
use chrono::{DateTime, Duration, Utc};
use serde_json::json;
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tracing::info;

use super::{
    UserForRegister,
    channel::{Channel, ChannelForCreate},
    log::Status,
    maintenance::{Maintenance, MaintenanceForSave},
    service::{Service, ServiceForCreate},
    status_page::{StatusPage, StatusPageForSave},
    user::{User, UserRole},
};

/// The shared account visitors sign in with; the login page fills it in.
pub const USERNAME: &str = "demo";
pub const PASSWORD: &str = "demo1234";

/// How much check history to make up, so charts and uptime aren't empty.
const HISTORY_HOURS: i64 = 24;

struct Sample {
    name: &'static str,
    interval: u16,
    tags: &'static [&'static str],
    config: serde_json::Value,
    active: bool,
    /// Typical response time in milliseconds.
    latency: u32,
    /// An outage in the history: (hours ago, minutes long).
    outage: Option<(i64, i64)>,
}

fn samples() -> Vec<Sample> {
    vec![
        Sample {
            name: "Stamon website",
            interval: 300,
            tags: &["website"],
            config: json!({ "type": "http", "url": "https://stamon.org/" }),
            active: true,
            latency: 120,
            outage: Some((7, 25)),
        },
        Sample {
            name: "GitHub",
            interval: 300,
            tags: &["third-party"],
            config: json!({ "type": "http", "url": "https://github.com/", "keyword": "GitHub" }),
            active: true,
            latency: 250,
            outage: None,
        },
        Sample {
            name: "GitHub SSH",
            interval: 300,
            tags: &["third-party"],
            config: json!({ "type": "tcp", "host": "github.com", "port": 22 }),
            active: true,
            latency: 60,
            outage: None,
        },
        Sample {
            name: "example.com DNS",
            interval: 300,
            tags: &["dns"],
            config: json!({
                "type": "dns",
                "host": "example.com",
                "record_type": "A",
                "resolver": "1.1.1.1"
            }),
            active: true,
            latency: 20,
            outage: Some((15, 10)),
        },
        Sample {
            name: "example.com certificate",
            interval: 3600,
            tags: &["certificates"],
            config: json!({ "type": "tls", "host": "example.com", "warn_days": 14 }),
            active: true,
            latency: 180,
            outage: None,
        },
        Sample {
            name: "Nightly backup",
            interval: 3600,
            tags: &["jobs"],
            config: json!({ "type": "push", "token": "" }),
            active: true,
            latency: 0,
            outage: None,
        },
        Sample {
            name: "Legacy API",
            interval: 300,
            tags: &["website"],
            config: json!({ "type": "http", "url": "https://example.com/" }),
            active: false,
            latency: 300,
            outage: None,
        },
    ]
}

/// Fill an empty database with the demo account and sample data. Does
/// nothing when there are users already, so a restart keeps visitors' data.
pub async fn seed(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let users: u32 = sqlx::query_scalar("SELECT COUNT(*) FROM Users")
        .fetch_one(pool)
        .await?;
    if users > 0 {
        return Ok(());
    }
    info!("Seeding the demo");

    // An editor rather than an admin, so visitors can't manage accounts.
    User::insert(
        pool,
        UserForRegister {
            username: USERNAME.into(),
            role: Some(UserRole::Editor),
            password: PASSWORD.into(),
            timezone: None,
        },
    )
    .await?;
    let user_id = User::find_by_username(pool, USERNAME)
        .await?
        .ok_or("demo user missing after insert")?
        .id;

    let now = Utc::now();
    let mut service_ids = vec![];
    for sample in samples() {
        let mut config: CheckConfig = serde_json::from_value(sample.config.clone())?;
        super::service::fill_push_token(&mut config);
        let id = Service::insert(
            pool,
            ServiceForCreate {
                user_id: Some(user_id),
                active: Some(sample.active),
                name: sample.name.into(),
                interval: sample.interval,
                timeout: Some(10),
                retry: 1,
                retry_interval: 0,
                invert: None,
                config,
                tags: sample.tags.iter().map(|t| t.to_string()).collect(),
                channel_ids: vec![],
            },
        )
        .await?;
        insert_history(pool, id, &sample, now).await?;
        service_ids.push(id);
    }

    Channel::insert(
        pool,
        ChannelForCreate {
            user_id: Some(user_id),
            name: "Team webhook".into(),
            active: Some(true),
            apply_to_all: Some(true),
            config: ChannelConfig::Webhook(WebhookConfig {
                url: "https://example.com/hooks/stamon".into(),
                headers: Default::default(),
            }),
        },
    )
    .await?;

    StatusPage::insert(
        pool,
        StatusPageForSave {
            user_id: Some(user_id),
            slug: "demo".into(),
            title: "Stamon demo".into(),
            description: Some("Sample services monitored by the Stamon demo.".into()),
            published: true,
            service_ids: service_ids[..4].to_vec(),
        },
    )
    .await?;

    let starts_at = now + Duration::days(1);
    Maintenance::insert(
        pool,
        MaintenanceForSave {
            user_id: Some(user_id),
            title: "Website redeploy".into(),
            description: Some("Planned downtime: these checks are paused.".into()),
            starts_at,
            ends_at: starts_at + Duration::hours(1),
            apply_to_all: false,
            service_ids: vec![service_ids[0]],
        },
    )
    .await?;

    Ok(())
}

/// Made-up results at the service's interval (at most every 5 minutes)
/// over the last day, oldest first so the latest sets the service's status.
async fn insert_history(
    pool: &SqlitePool,
    service_id: u32,
    sample: &Sample,
    now: DateTime<Utc>,
) -> sqlx::Result<()> {
    if !sample.active {
        return Ok(());
    }
    let step = i64::from(sample.interval).max(300);
    let checks = HISTORY_HOURS * 3600 / step;
    let logs = (1..=checks).rev().map(|i| {
        let time = now - Duration::seconds(i * step);
        let down = sample.outage.is_some_and(|(hours_ago, minutes)| {
            let start = now - Duration::hours(hours_ago);
            time >= start && time < start + Duration::minutes(minutes)
        });
        // Vary the latency a little so the chart has some shape.
        let jitter = (i * 37 % 41) as u32;
        if down {
            (
                Status::Down,
                Some("Unexpected status code: 503 Service Unavailable"),
                time,
                sample.latency * 3 + jitter,
            )
        } else {
            (
                Status::Up,
                None,
                time,
                sample.latency + sample.latency * jitter / 100,
            )
        }
    });

    // One statement: a row at a time makes startup slow.
    let mut query = QueryBuilder::<Sqlite>::new(
        "INSERT INTO Logs (service_id, status, message, time, duration) ",
    );
    query.push_values(logs, |mut row, (status, message, time, duration)| {
        row.push_bind(service_id)
            .push_bind(status)
            .push_bind(message)
            .push_bind(time)
            .push_bind(duration);
    });
    query.build().execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_are_valid_and_public() {
        for sample in samples() {
            let mut config: CheckConfig = serde_json::from_value(sample.config.clone()).unwrap();
            super::super::service::fill_push_token(&mut config);
            config.validate().unwrap();
            config.validate_public().unwrap();
        }
    }

    #[sqlx::test]
    async fn seeds_once(pool: SqlitePool) {
        seed(&pool).await.unwrap();
        let user = User::find_by_username(&pool, USERNAME)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.role, UserRole::Editor);
        let services = Service::count(&pool).await.unwrap();
        assert_eq!(services, samples().len() as u32);

        seed(&pool).await.unwrap();
        assert_eq!(Service::count(&pool).await.unwrap(), services);
    }
}
