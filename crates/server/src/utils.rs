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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_shutdown_signal_timeout() {
        // Test that shutdown_signal doesn't complete immediately
        // We use a short timeout to ensure the test doesn't hang
        let result = timeout(Duration::from_millis(10), shutdown_signal()).await;
        
        // Should timeout since no signal was sent
        assert!(result.is_err());
    }

    #[test]
    fn test_shutdown_signal_exists() {
        // Test that the function exists and can be called
        // This is a basic compilation test
        let _future = shutdown_signal();
        // If this compiles, the function signature is correct
    }
}
