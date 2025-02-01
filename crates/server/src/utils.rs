use std::io;

use tokio::signal;

pub async fn shutdown_signal() -> io::Result<()> {
    let ctrl_c = signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        v = ctrl_c => v,
        _ = terminate => Ok(()),
    }
}
