use std::time::Duration;

use apalis::prelude::{Context, Data, Worker};
use checks::{CheckOutcome, CheckStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::{
    AppState,
    models::{
        log::{Log, LogForCreate, Status},
        service::Service,
    },
    ws::{Event, Level, Notification},
};

/// A queued check. The service is loaded when the job runs, so it always uses
/// the latest settings and retry state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckJob {
    pub service_id: u32,
}

fn to_log(service_id: u32, time: DateTime<Utc>, outcome: CheckOutcome) -> LogForCreate {
    LogForCreate {
        service_id,
        status: match outcome.status {
            CheckStatus::Up => Status::Up,
            CheckStatus::Down => Status::Down,
            CheckStatus::Error => Status::Failed,
        },
        message: outcome.message,
        time: Some(time),
        duration: outcome.latency.as_millis().try_into().unwrap_or(u32::MAX),
    }
}

/// Result of applying a service's invert and retry settings to a raw check.
#[derive(Debug)]
struct Evaluation {
    log: LogForCreate,
    consecutive_failures: u32,
    /// The failure isn't confirmed yet; check again after `retry_interval`.
    retrying: bool,
}

fn evaluate(svc: &Service, mut log: LogForCreate) -> Evaluation {
    if svc.invert {
        match log.status {
            Status::Up => {
                log.status = Status::Down;
                log.message = Some("Service responded but the check is inverted".into());
            }
            Status::Down => {
                log.status = Status::Up;
                log.message = None;
            }
            _ => (),
        }
    }

    let mut retrying = false;
    let consecutive_failures = match log.status {
        Status::Up => 0,
        Status::Down => {
            let failures = svc.consecutive_failures.saturating_add(1);
            // A service that is already down doesn't need confirming again.
            if !matches!(svc.last_status, Status::Down) && failures <= svc.retry {
                retrying = true;
                log.status = Status::Pending;
                log.message = Some(format!(
                    "Retry {failures}/{}: {}",
                    svc.retry,
                    log.message.as_deref().unwrap_or("check failed")
                ));
            }
            failures
        }
        // Monitor errors say nothing about the service itself.
        Status::Pending | Status::Failed => svc.consecutive_failures,
    };

    Evaluation {
        log,
        consecutive_failures,
        retrying,
    }
}

fn transition_notification(name: &str, from: Status, to: Status) -> Option<Notification> {
    match (from, to) {
        (Status::Down, Status::Up) => Some(Notification {
            message: format!("Service {name} back Up"),
            title: "Back Up".to_string(),
            level: Level::Success,
        }),
        (Status::Failed, Status::Up | Status::Down) => Some(Notification {
            message: format!("Service {name} check success"),
            title: "Monitor Success".to_string(),
            level: Level::Info,
        }),
        (Status::Up | Status::Pending, Status::Down) => Some(Notification {
            message: format!("Service {name} is Down"),
            title: "Service Down".to_string(),
            level: Level::Warning,
        }),
        _ => None,
    }
}

pub async fn job_monitor(job: CheckJob, worker: Worker<Context>, state: Data<AppState>) {
    let svc = match Service::get(&state.pool, job.service_id).await {
        Ok(Some(svc)) if svc.active => svc,
        Ok(_) => {
            debug!(
                worker = worker.id().to_string(),
                "Service {} removed or paused", job.service_id
            );
            return;
        }
        Err(e) => {
            error!("Failed to load service {}: {e}", job.service_id);
            return;
        }
    };

    let time = Utc::now();
    let outcome = svc
        .config
        .check(Duration::from_secs(svc.timeout.max(1).into()))
        .await;
    if outcome.status == CheckStatus::Error
        && let Err(e) = state.tx.send(Event::Notification(Notification {
            message: format!(
                "Could not check {}: {}",
                svc.name,
                outcome.message.as_deref().unwrap_or("unknown error")
            ),
            title: "Monitor Error".to_string(),
            level: Level::Error,
        }))
    {
        error!("Failed to send notification: {:?}", e);
    }
    let checked = to_log(svc.id, time, outcome);
    let Evaluation {
        log: status_log,
        consecutive_failures,
        retrying,
    } = evaluate(&svc, checked);

    if let Err(e) = state.tx.send(Event::Log(status_log.clone())) {
        error!("Failed to send notification: {:?}", e);
    }
    debug!(
        worker = worker.id().to_string(),
        "Service status {}", status_log
    );

    if let Some(notification) =
        transition_notification(&svc.name, svc.last_status, status_log.status)
        && let Err(e) = state.tx.send(Event::Notification(notification))
    {
        error!("Failed to send notification: {:?}", e);
    }

    if let Err(e) = Log::insert(&state.pool, status_log).await {
        error!("error {e}");
    };

    if let Err(e) =
        Service::set_consecutive_failures(&state.pool, svc.id, consecutive_failures).await
    {
        error!("Failed to update failure count for service {}: {e}", svc.id);
    }
    if retrying {
        let delay = if svc.retry_interval > 0 {
            svc.retry_interval
        } else {
            svc.interval
        };
        let next_run_at = Utc::now().timestamp() + i64::from(delay.max(1));
        if let Err(e) = Service::set_next_run(&state.pool, svc.id, next_run_at).await {
            error!("Failed to schedule retry for service {}: {e}", svc.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use checks::{CheckConfig, PingConfig};

    use super::*;

    fn service(url: &str) -> Service {
        Service {
            id: 1,
            user_id: 1,
            active: true,
            name: "test".into(),
            interval: 60,
            timeout: 5,
            last_status: Status::Up,
            retry: 0,
            retry_interval: 0,
            invert: false,
            consecutive_failures: 0,
            next_run_at: 0,
            config: CheckConfig::Ping(PingConfig { host: url.into() }),
            service_type: "ping".into(),
            target: url.into(),
        }
    }

    #[test]
    fn outcome_maps_to_log() {
        let time = Utc::now();
        let log = to_log(
            7,
            time,
            CheckOutcome {
                status: CheckStatus::Error,
                latency: Duration::from_millis(1500),
                message: Some("no permission".into()),
            },
        );
        assert!(matches!(log.status, Status::Failed));
        assert_eq!(log.service_id, 7);
        assert_eq!(log.duration, 1500);
        assert_eq!(log.time, Some(time));
        assert_eq!(log.message.as_deref(), Some("no permission"));
    }

    fn log(status: Status) -> LogForCreate {
        LogForCreate {
            status,
            message: Some("boom".into()),
            ..Default::default()
        }
    }

    #[test]
    fn invert_flips_up_and_down() {
        let mut svc = service("x");
        svc.invert = true;

        let up = evaluate(&svc, log(Status::Down));
        assert!(matches!(up.log.status, Status::Up));
        assert_eq!(up.consecutive_failures, 0);

        let down = evaluate(&svc, log(Status::Up));
        assert!(matches!(down.log.status, Status::Down));

        let failed = evaluate(&svc, log(Status::Failed));
        assert!(matches!(failed.log.status, Status::Failed));
    }

    #[test]
    fn retries_before_marking_down() {
        let mut svc = service("x");
        svc.retry = 2;

        let first = evaluate(&svc, log(Status::Down));
        assert!(matches!(first.log.status, Status::Pending));
        assert!(first.retrying);
        assert_eq!(first.consecutive_failures, 1);
        assert_eq!(first.log.message.as_deref(), Some("Retry 1/2: boom"));

        svc.last_status = Status::Pending;
        svc.consecutive_failures = 2;
        let confirmed = evaluate(&svc, log(Status::Down));
        assert!(matches!(confirmed.log.status, Status::Down));
        assert!(!confirmed.retrying);
        assert_eq!(confirmed.consecutive_failures, 3);
    }

    #[test]
    fn no_retry_when_already_down_or_disabled() {
        let mut svc = service("x");
        assert!(matches!(
            evaluate(&svc, log(Status::Down)).log.status,
            Status::Down
        ));

        svc.retry = 3;
        svc.last_status = Status::Down;
        assert!(matches!(
            evaluate(&svc, log(Status::Down)).log.status,
            Status::Down
        ));
    }

    #[test]
    fn up_resets_failures() {
        let mut svc = service("x");
        svc.consecutive_failures = 4;
        assert_eq!(evaluate(&svc, log(Status::Up)).consecutive_failures, 0);
    }

    #[test]
    fn notifies_on_transitions() {
        let n = |from, to| transition_notification("svc", from, to).map(|n| n.title);
        assert_eq!(n(Status::Up, Status::Down).as_deref(), Some("Service Down"));
        assert_eq!(
            n(Status::Pending, Status::Down).as_deref(),
            Some("Service Down")
        );
        assert_eq!(n(Status::Down, Status::Up).as_deref(), Some("Back Up"));
        assert!(n(Status::Up, Status::Pending).is_none());
        assert!(n(Status::Pending, Status::Up).is_none());
        assert!(n(Status::Down, Status::Down).is_none());
    }
}
