use crate::error::service_util_error::ServiceUtilError;
use crate::ServiceUtil;
use std::process::Command;
use std::time::Duration;
use tokio::time::Instant;

impl ServiceUtil {
    /// Waits until the health check URL responds successfully.
    ///
    /// # Arguments
    ///
    /// * `health_url` - The URL to ping for health check.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if the healthcheck times out.
    ///
    pub(crate) fn wait_until_http_health_check(
        &self,
        health_url: &str,
        timeout: &Duration,
    ) -> Result<(), ServiceUtilError> {
        let start_time = Instant::now();

        loop {
            std::thread::sleep(Duration::from_millis(100));

            if start_time.elapsed().as_secs() > timeout.as_secs() {
                return Err(ServiceUtilError::ServiceHealthcheckFailed(format!(
                    "[wait_until_http_health_check]: !!Timeout!! Waited {} seconds for service health check",
                    timeout.as_secs(),
                )));
            }

            let mut cmd = Command::new("curl");
            cmd.arg(health_url);

            if let Ok(out) = cmd.output() {
                self.dbg_print(&format!(
                    "[wait_until_http_health_check]: \n
                    success: {} \n
                    Output: {}",
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                ));

                if out.status.success() {
                    self.dbg_print("Service online");

                    break Ok(());
                }
            }
        }
    }
}
