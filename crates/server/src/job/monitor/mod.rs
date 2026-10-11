use apalis::prelude::{Context, Data, Worker};
use chrono::Utc;
use tracing::{debug, error};

use crate::{
    AppState,
    models::{
        log::{Log, LogForCreate, Status},
        service::{Service, ServiceType},
    },
    ws::{Event, Level, Notification},
};

mod http;
mod ping;

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

pub async fn job_monitor(job: Service, worker: Worker<Context>, state: Data<AppState>) {
    // The queued job is a snapshot. Reload it so edits, pauses, deletes and the
    // retry state made since it was queued are respected.
    let svc = match Service::get(&state.pool, job.id).await {
        Ok(Some(svc)) if svc.active => svc,
        Ok(_) => {
            debug!(
                worker = worker.id().to_string(),
                "Service {} removed or paused", job.id
            );
            return;
        }
        Err(e) => {
            error!("Failed to reload service {}: {e}", job.id);
            job
        }
    };

    let checked = match svc.service_type {
        ServiceType::Ping => ping::ping(svc.clone(), state.tx.clone()).await,
        ServiceType::Http => http::get(svc.clone(), state.tx.clone()).await,
    };
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
pub(super) mod tests {
    use super::*;

    pub fn service(url: &str) -> Service {
        Service {
            id: 1,
            user_id: 1,
            active: true,
            name: "test".into(),
            interval: 60,
            url: url.into(),
            timeout: 5,
            payload: None,
            last_status: Status::Up,
            service_type: ServiceType::Http,
            retry: 0,
            retry_interval: 0,
            invert: false,
            expected_code: None,
            expected_payload: None,
            consecutive_failures: 0,
            next_run_at: 0,
        }
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
