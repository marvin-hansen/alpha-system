use common_config::prelude::ServiceID;
use std::process::Command;
use std::time::Duration;
use tokio::time::{sleep, Instant};

use crate::error::service_util_error::ServiceUtilError;
use crate::fields::PATH;
use crate::{ServiceUtil, ServiceWaitStrategy};

impl ServiceUtil {
    /// Starts a service with the given ID.
    ///
    /// # Arguments
    ///
    /// * `svc` - The service ID.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if the service could not be started.
    pub async fn start_service(
        &self,
        svc: &ServiceID,
        wait_strategy: &ServiceWaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        self.dbg_print("start_service");
        self.dbg_print(&format!(
            "Starting service: {}",
            svc.to_string().to_lowercase()
        ));

        self.dbg_print("Setting program to executable");
        let program = format!("{}/{}", PATH, svc.to_string().to_lowercase());
        let mut cmd = Command::new("chmod");
        cmd.arg("+x").arg(program.clone());
        cmd.output().expect("Failed to set program to executable");

        self.dbg_print("Constructing start command");
        let mut cmd = Command::new(program);
        cmd.arg("&");

        self.dbg_print("Setting environment variables");
        let (env, val) = self.config_manager.env_var();
        cmd.env(env, val);

        self.dbg_print(&format!("Run start command: {:?}", &cmd));
        cmd.spawn().expect("Failed to run command");

        self.dbg_print("Waiting for service to start");
        match wait_strategy {
            ServiceWaitStrategy::Duration(duration) => {
                self.wait_until_timeout(duration)
                    .await
                    .expect("Failed to wait");
            }
            ServiceWaitStrategy::HttpHealthCheck(health_url, duration) => {
                self.wait_until_http_health_check(&health_url, duration)
                    .expect("Failed to wait for health check");
            }
        }
        self.dbg_print("Service started");

        Ok(())
    }

    async fn wait_until_timeout(&self, wait_duration: &Duration) -> Result<(), ServiceUtilError> {
        sleep(wait_duration.to_owned()).await;
        Ok(())
    }

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
    pub fn wait_until_http_health_check(
        &self,
        health_url: &str,
        timeout: &Duration,
    ) -> Result<(), ServiceUtilError> {
        let start_time = Instant::now();

        loop {
            std::thread::sleep(Duration::from_millis(100));

            if start_time.elapsed().as_secs() > timeout.as_secs() {
                return Err(ServiceUtilError::ServiceHealthcheckFailed(format!(
                    "[start_service]: !!Timeout!! Waited {} seconds for service to respond to health check",
                    timeout.as_secs(),
                )));
            }

            let mut cmd = Command::new("curl");
            cmd.arg(health_url);

            if let Ok(out) = cmd.output() {
                self.dbg_print(&format!(
                    "[wait_until_health_check]: \n
                    success: {} \n
                    Output: {}",
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                ));

                if out.status.success() && String::from_utf8_lossy(&out.stdout).contains("OK") {
                    self.dbg_print("Service online");

                    break Ok(());
                }
            }
        }
    }
}
