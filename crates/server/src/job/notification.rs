use apalis::prelude::{Data, WorkerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub to: String,
    pub text: String,
}

pub async fn notify(job: Notification, wid: Data<WorkerId>) {
    tracing::info!(
        worker = wid.to_string(),
        "Attempting to send notification to {}",
        job.to
    );
}
