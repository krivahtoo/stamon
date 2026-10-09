use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use super::{
    log::{Log, Status},
    maintenance::Maintenance,
};

/// Days of history shown per service on a public page.
const HISTORY_DAYS: i64 = 30;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct StatusPage {
    pub id: u32,
    pub user_id: u32,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub published: bool,
    /// Services on the page, in order.
    #[sqlx(skip)]
    pub service_ids: Vec<u32>,
}

/// A whole page, used to create one or replace an existing one.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusPageForSave {
    #[serde(skip)]
    pub user_id: Option<u32>,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub published: bool,
    #[serde(default)]
    pub service_ids: Vec<u32>,
}

impl StatusPageForSave {
    pub fn validate(&self) -> Result<(), String> {
        let slug = &self.slug;
        let valid_slug = !slug.is_empty()
            && slug.len() <= 64
            && slug.split('-').all(|part| {
                !part.is_empty()
                    && part
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            });
        if !valid_slug {
            return Err("slug may only contain lowercase letters, digits and single dashes".into());
        }
        if self.title.trim().is_empty() {
            return Err("title must not be empty".into());
        }
        Ok(())
    }
}

/// The state of everything on a page at a glance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OverallStatus {
    Operational,
    /// Some services are down.
    Degraded,
    /// Every service is down.
    Outage,
    /// Nothing is down, but some services are in maintenance.
    Maintenance,
}

pub fn overall_status(statuses: &[Status]) -> OverallStatus {
    let down = statuses
        .iter()
        .filter(|s| matches!(s, Status::Down))
        .count();
    if down > 0 && down == statuses.len() {
        OverallStatus::Outage
    } else if down > 0 {
        OverallStatus::Degraded
    } else if statuses.iter().any(|s| matches!(s, Status::Maintenance)) {
        OverallStatus::Maintenance
    } else {
        OverallStatus::Operational
    }
}

#[derive(Debug, Serialize)]
pub struct DayUptime {
    pub date: NaiveDate,
    /// `None` for days without results.
    pub uptime: Option<f64>,
}

/// What a public page shows about a service: no URLs, hosts or settings.
#[derive(Debug, Serialize)]
pub struct PublicService {
    pub name: String,
    pub status: Status,
    pub uptime: Option<f64>,
    /// The last 30 days, oldest first.
    pub days: Vec<DayUptime>,
}

#[derive(Debug, Serialize)]
pub struct PublicMaintenance {
    pub title: String,
    pub description: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PublicStatusPage {
    pub title: String,
    pub description: Option<String>,
    pub status: OverallStatus,
    pub services: Vec<PublicService>,
    /// Current and upcoming maintenance for the page's services.
    pub maintenance: Vec<PublicMaintenance>,
    pub generated_at: DateTime<Utc>,
}

/// One entry per day from `first` to `last`, filling days without results.
fn fill_days(
    first: NaiveDate,
    last: NaiveDate,
    counts: &[(NaiveDate, u32, u32)],
) -> Vec<DayUptime> {
    first
        .iter_days()
        .take_while(|date| *date <= last)
        .map(|date| DayUptime {
            date,
            uptime: counts
                .iter()
                .find(|(day, _, _)| *day == date)
                .map(|(_, up, down)| f64::from(*up) * 100.0 / f64::from(up + down)),
        })
        .collect()
}

impl StatusPage {
    async fn with_service_ids(
        pool: &SqlitePool,
        mut pages: Vec<StatusPage>,
    ) -> sqlx::Result<Vec<StatusPage>> {
        let links: Vec<(u32, u32)> = sqlx::query_as(
            "SELECT page_id, service_id FROM StatusPageServices ORDER BY page_id, position",
        )
        .fetch_all(pool)
        .await?;
        for page in &mut pages {
            page.service_ids = links
                .iter()
                .filter(|(page_id, _)| *page_id == page.id)
                .map(|(_, service_id)| *service_id)
                .collect();
        }
        Ok(pages)
    }

    pub async fn list(pool: &SqlitePool) -> sqlx::Result<Vec<StatusPage>> {
        let pages = sqlx::query_as("SELECT * FROM StatusPages ORDER BY title")
            .fetch_all(pool)
            .await?;
        Self::with_service_ids(pool, pages).await
    }

    pub async fn get_by_slug(pool: &SqlitePool, slug: &str) -> sqlx::Result<Option<StatusPage>> {
        let page = sqlx::query_as("SELECT * FROM StatusPages WHERE slug = ?")
            .bind(slug)
            .fetch_optional(pool)
            .await?;
        let pages = Self::with_service_ids(pool, page.into_iter().collect()).await?;
        Ok(pages.into_iter().next())
    }

    async fn save_services(
        tx: &mut sqlx::SqliteConnection,
        page_id: u32,
        service_ids: &[u32],
    ) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM StatusPageServices WHERE page_id = ?")
            .bind(page_id)
            .execute(&mut *tx)
            .await?;
        for (position, service_id) in service_ids.iter().enumerate() {
            sqlx::query(
                r#"INSERT OR IGNORE INTO StatusPageServices (page_id, service_id, position)
                   VALUES (?, ?, ?)"#,
            )
            .bind(page_id)
            .bind(service_id)
            .bind(position as u32)
            .execute(&mut *tx)
            .await?;
        }
        Ok(())
    }

    /// Returns the new page's id.
    pub async fn insert(pool: &SqlitePool, page: StatusPageForSave) -> sqlx::Result<u32> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query(
            r#"INSERT INTO StatusPages (user_id, slug, title, description, published)
               VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(page.user_id)
        .bind(&page.slug)
        .bind(page.title.trim())
        .bind(page.description.filter(|d| !d.trim().is_empty()))
        .bind(page.published)
        .execute(&mut *tx)
        .await?;
        let id = result.last_insert_rowid() as u32;
        Self::save_services(&mut tx, id, &page.service_ids).await?;
        tx.commit().await?;
        Ok(id)
    }

    /// Replace a page's details and services; returns whether it existed.
    pub async fn replace(
        pool: &SqlitePool,
        page_id: u32,
        page: StatusPageForSave,
    ) -> sqlx::Result<bool> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query(
            r#"UPDATE StatusPages
               SET slug = ?, title = ?, description = ?, published = ?
               WHERE id = ?"#,
        )
        .bind(&page.slug)
        .bind(page.title.trim())
        .bind(page.description.filter(|d| !d.trim().is_empty()))
        .bind(page.published)
        .bind(page_id)
        .execute(&mut *tx)
        .await?;
        if result.rows_affected() == 0 {
            return Ok(false);
        }
        Self::save_services(&mut tx, page_id, &page.service_ids).await?;
        tx.commit().await?;
        Ok(true)
    }

    pub async fn delete(pool: &SqlitePool, page_id: u32) -> sqlx::Result<u64> {
        let result = sqlx::query("DELETE FROM StatusPages WHERE id = ?")
            .bind(page_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// What the public sees, leaving out paused services.
    pub async fn public_view(
        &self,
        pool: &SqlitePool,
        now: DateTime<Utc>,
    ) -> sqlx::Result<PublicStatusPage> {
        let since = now - TimeDelta::days(HISTORY_DAYS - 1);
        let first_day = since.date_naive();
        let since = first_day.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc();

        let mut services = Vec::new();
        let mut maintenance: Vec<Maintenance> = Vec::new();
        for service_id in &self.service_ids {
            let row: Option<(String, Status, bool)> =
                sqlx::query_as("SELECT name, last_status, active FROM Services WHERE id = ?")
                    .bind(service_id)
                    .fetch_optional(pool)
                    .await?;
            let Some((name, status, true)) = row else {
                continue;
            };
            let counts = Log::daily_counts(pool, *service_id, since).await?;
            services.push(PublicService {
                name,
                status,
                uptime: Log::uptime(pool, *service_id, since).await?.percent,
                days: fill_days(first_day, now.date_naive(), &counts),
            });
            for window in Maintenance::upcoming_for_service(pool, *service_id, now).await? {
                if !maintenance.iter().any(|w| w.id == window.id) {
                    maintenance.push(window);
                }
            }
        }
        maintenance.sort_by_key(|w| w.starts_at);

        let statuses: Vec<Status> = services.iter().map(|s| s.status).collect();
        Ok(PublicStatusPage {
            title: self.title.clone(),
            description: self.description.clone(),
            status: overall_status(&statuses),
            services,
            maintenance: maintenance
                .into_iter()
                .map(|w| PublicMaintenance {
                    title: w.title,
                    description: w.description,
                    starts_at: w.starts_at,
                    ends_at: w.ends_at,
                })
                .collect(),
            generated_at: now,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{log::LogForCreate, maintenance::MaintenanceForSave};

    fn page(slug: &str, published: bool, service_ids: &[u32]) -> StatusPageForSave {
        StatusPageForSave {
            user_id: Some(1),
            slug: slug.into(),
            title: "Acme status".into(),
            description: None,
            published,
            service_ids: service_ids.to_vec(),
        }
    }

    #[test]
    fn overall_status_summarises_services() {
        use Status::*;
        assert_eq!(overall_status(&[Up, Up]), OverallStatus::Operational);
        assert_eq!(overall_status(&[]), OverallStatus::Operational);
        assert_eq!(overall_status(&[Up, Pending]), OverallStatus::Operational);
        assert_eq!(overall_status(&[Up, Down]), OverallStatus::Degraded);
        assert_eq!(overall_status(&[Down, Down]), OverallStatus::Outage);
        assert_eq!(
            overall_status(&[Up, Maintenance]),
            OverallStatus::Maintenance
        );
        assert_eq!(
            overall_status(&[Down, Maintenance]),
            OverallStatus::Degraded
        );
    }

    #[test]
    fn slug_validation() {
        for slug in ["acme", "acme-status", "v2-api"] {
            assert!(page(slug, true, &[]).validate().is_ok(), "{slug}");
        }
        for slug in ["", "Acme", "acme--status", "-acme", "acme status", "acme/x"] {
            assert!(page(slug, true, &[]).validate().is_err(), "{slug}");
        }
    }

    #[test]
    fn missing_days_are_filled() {
        let day = |d| NaiveDate::from_ymd_opt(2026, 10, d).unwrap();
        let days = fill_days(day(1), day(3), &[(day(2), 3, 1)]);
        let uptimes: Vec<_> = days.iter().map(|d| (d.date, d.uptime)).collect();
        assert_eq!(
            uptimes,
            [(day(1), None), (day(2), Some(75.0)), (day(3), None)]
        );
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "services")))]
    async fn save_and_order_services(pool: SqlitePool) -> sqlx::Result<()> {
        let id = StatusPage::insert(&pool, page("acme", false, &[4, 1, 2])).await?;
        let saved = StatusPage::get_by_slug(&pool, "acme").await?.unwrap();
        assert_eq!(saved.id, id);
        assert_eq!(saved.service_ids, [4, 1, 2], "kept in the given order");

        assert!(StatusPage::replace(&pool, id, page("acme-v2", true, &[2])).await?);
        assert!(StatusPage::get_by_slug(&pool, "acme").await?.is_none());
        let saved = StatusPage::get_by_slug(&pool, "acme-v2").await?.unwrap();
        assert!(saved.published);
        assert_eq!(saved.service_ids, [2]);

        // Slugs are unique.
        assert!(
            StatusPage::insert(&pool, page("acme-v2", true, &[]))
                .await
                .is_err()
        );

        assert_eq!(StatusPage::list(&pool).await?.len(), 1);
        assert_eq!(StatusPage::delete(&pool, id).await?, 1);
        assert!(StatusPage::list(&pool).await?.is_empty());
        Ok(())
    }

    #[sqlx::test(fixtures(path = "fixtures", scripts("users", "services")))]
    async fn public_view_hides_details(pool: SqlitePool) -> sqlx::Result<()> {
        let now = Utc::now();
        // Service 3 is paused in the fixtures.
        StatusPage::insert(&pool, page("acme", true, &[1, 3, 2])).await?;
        for status in [Status::Up, Status::Up, Status::Up, Status::Down] {
            Log::insert(
                &pool,
                LogForCreate {
                    service_id: 1,
                    status,
                    message: None,
                    time: Some(now),
                    duration: 10,
                },
            )
            .await?;
        }
        Maintenance::insert(
            &pool,
            MaintenanceForSave {
                user_id: Some(1),
                title: "Upgrade".into(),
                description: None,
                starts_at: now + TimeDelta::hours(1),
                ends_at: now + TimeDelta::hours(2),
                apply_to_all: false,
                service_ids: vec![2],
            },
        )
        .await?;

        let page = StatusPage::get_by_slug(&pool, "acme").await?.unwrap();
        let view = page.public_view(&pool, now).await?;
        let names: Vec<&str> = view.services.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(
            names,
            ["Service One", "Service Two"],
            "paused service hidden"
        );
        assert_eq!(view.services[0].uptime, Some(75.0));
        assert_eq!(view.services[0].days.len(), HISTORY_DAYS as usize);
        assert_eq!(view.services[0].days.last().unwrap().uptime, Some(75.0));
        assert_eq!(view.maintenance.len(), 1);
        assert_eq!(view.maintenance[0].title, "Upgrade");

        // The trigger sets Service One to Down from its last log.
        assert_eq!(view.status, OverallStatus::Degraded);

        let json = serde_json::to_string(&view).unwrap();
        assert!(!json.contains("service-one.com"), "targets stay private");
        Ok(())
    }
}
