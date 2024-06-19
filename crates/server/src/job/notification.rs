use apalis::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub to: String,
    pub text: String,
}

impl Job for Notification {
    const NAME: &'static str = "stamon::Notification";
}

pub async fn notify(job: Notification, wid: Data<WorkerId>) {
    tracing::info!(
        worker = wid.to_string(),
        "Attempting to send notification to {}",
        job.to
    );
}
