use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub to: String,
    pub text: String,
}

// Taking the worker here (`Data<WorkerId>` or `Worker<Context>`) breaks the
// notification worker under apalis 0.7, so it is left out; the worker's trace
// span already identifies it.
pub async fn notify(job: Notification) {
    tracing::info!("Attempting to send notification to {}", job.to);
}
