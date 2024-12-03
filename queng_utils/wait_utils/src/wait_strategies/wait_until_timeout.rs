use crate::WaitStrategyError;
use std::time::Duration;
use tokio::time::sleep;

/// Waits for the given duration asynchronously.
///
/// # Arguments
///
/// * `wait_duration` - The duration to wait for.
///
/// # Returns
///
/// Returns Ok(()) when the wait is complete.
///
pub async fn wait_until_timeout(wait_duration: &Duration) -> Result<(), WaitStrategyError> {
    sleep(wait_duration.to_owned()).await;
    Ok(())
}
