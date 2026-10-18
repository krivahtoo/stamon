use std::time::Duration;

use alerts::Alert;
use apalis::prelude::Data;
use serde::{Deserialize, Serialize};
use tracing::{error, warn};

use crate::{
    AppState,
    config::env_config,
    models::{
        channel::Channel,
        notification::{DeliveryStatus, Notification, NotificationForCreate},
    },
};

/// Attempts per alert before it's recorded as failed.
const ATTEMPTS: u32 = 3;

/// An alert waiting to be sent to one channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertJob {
    pub channel_id: u32,
    pub alert: Alert,
}

/// Send an alert, retrying with backoff, and record how it went. The channel
/// is loaded now so a channel edited, paused or deleted meanwhile is respected.
pub async fn deliver(job: AlertJob, state: Data<AppState>) {
    let channel = match Channel::get(&state.pool, job.channel_id).await {
        Ok(Some(channel)) if channel.active => channel,
        Ok(_) => return,
        Err(e) => {
            error!("Failed to load channel {}: {e}", job.channel_id);
            return;
        }
    };

    // Anyone can add services to the demo, so it never sends alerts.
    let error = if env_config().demo {
        Some("Alerts aren't sent in the demo".to_string())
    } else {
        send(&channel, &job.alert).await
    };

    let notification = NotificationForCreate {
        service_id: Some(job.alert.service_id),
        channel_id: Some(channel.id),
        title: job.alert.title(),
        message: job.alert.message.clone(),
        status: if error.is_none() {
            DeliveryStatus::Sent
        } else {
            DeliveryStatus::Failed
        },
        error,
    };
    if let Err(e) = Notification::insert(&state.pool, notification).await {
        error!("Failed to record alert delivery: {e}");
    }
}

/// Send with retries; returns the last error if every attempt failed.
async fn send(channel: &Channel, alert: &Alert) -> Option<String> {
    let mut error = None;
    for attempt in 1..=ATTEMPTS {
        match channel.config.send(alert).await {
            Ok(()) => return None,
            Err(e) => {
                warn!(
                    "Alert to channel {} failed (attempt {attempt}/{ATTEMPTS}): {e}",
                    channel.id
                );
                error = Some(e);
                if attempt < ATTEMPTS {
                    tokio::time::sleep(Duration::from_secs(2u64.pow(attempt))).await;
                }
            }
        }
    }
    error
}
