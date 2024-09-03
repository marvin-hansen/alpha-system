use std::process::Command;
use std::time::{Duration, Instant};

use common_config::prelude::ServiceID;

use crate::error::service_util_error::ServiceUtilError;
use crate::fields::PATH;
use crate::ServiceUtil;

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
    pub async fn start_service(&self, svc: &ServiceID) -> Result<(), ServiceUtilError> {
        self.dbg_print("start_service");
        self.dbg_print(&format!(
            "Starting service: {}",
            svc.to_string().to_lowercase()
        ));

        let program = format!("{}/{}", PATH, svc.to_string().to_lowercase());
        let health_url = match self.config_manager.get_health_check_url(svc).await {
            Ok(uri) => uri,
            Err(e) => return Err(ServiceUtilError::UnknownError(e.to_string())),
        };
        self.dbg_print(&health_url);

        self.dbg_print("Setting program to executable");
        let mut cmd = Command::new("chmod");
        cmd.arg("+x").arg(program.clone());
        cmd.output().expect("Failed to set program to executable");

        self.dbg_print("Constructing initial command");
        let mut cmd = Command::new(program);
        cmd.arg("&");

        self.dbg_print("Setting environment variables");
        let (env, val) = self.config_manager.env_var();
        cmd.env(env, val);

        self.dbg_print(&format!("Run command: {:?}", &cmd));
        cmd.spawn().expect("Failed to run command");

        self.dbg_print("Waiting for service to start");
        self.wait_until_health_check(&health_url)
            .expect("Failed to wait for service to start");

        self.dbg_print("Service started");

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
    pub fn wait_until_health_check(&self, health_url: &str) -> Result<(), ServiceUtilError> {
        let start_time = Instant::now();
        let timeout = Duration::from_secs(12);

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

            match cmd.output() {
                Ok(out) => {
                    self.dbg_print(&format!(
                        "[wait_until_health_check]: \n
                        success: {} \n
                        Output: {}",
                        out.status.success(),
                        String::from_utf8_lossy(out.stdout.as_slice()),
                    ));

                    if out.status.success() {
                        if String::from_utf8_lossy(&out.stdout).contains("OK") {
                            self.dbg_print("Service online");

                            break Ok(());
                        }
                    }
                }
                Err(_) => {} // ignore as curl returns an error in case connection failure
                             // Instead, try again until curl either receives an OK response or times out
            }
        }
    }
}
