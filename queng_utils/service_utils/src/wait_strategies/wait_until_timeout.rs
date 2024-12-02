use crate::error::service_util_error::ServiceUtilError;
use crate::ServiceUtil;
use std::time::Duration;
use tokio::time::sleep;

impl ServiceUtil {
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
    pub(crate) async fn wait_until_timeout(
        &self,
        wait_duration: &Duration,
    ) -> Result<(), ServiceUtilError> {
        sleep(wait_duration.to_owned()).await;
        Ok(())
    }
}
