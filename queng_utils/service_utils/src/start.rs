use crate::error::service_util_error::ServiceUtilError;
use crate::{ServiceUtil, ServiceWaitStrategy};
use std::process::Command;

impl ServiceUtil {
    //
    /// Starts a program with the specified wait strategy.
    ///
    /// # Arguments
    ///
    /// * `program` - The program to start.
    /// * `wait_strategy` - How to wait for the service to become ready.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if the program could not be started.
    ///
    pub(crate) async fn start_program(
        &self,
        program: String,
        wait_strategy: &ServiceWaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        // Set the program to be executable
        Command::new("chmod")
            .arg("+x")
            .arg(&program)
            .output()
            .expect("Failed to set program to executable");
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
                self.wait_until_http_health_check(health_url, duration)
                    .expect("Failed to wait for health check");
            }
        }
        self.dbg_print("Service started");
        Ok(())
    }
}
