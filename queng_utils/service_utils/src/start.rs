use crate::error::service_util_error::ServiceUtilError;
use crate::ServiceUtil;
use std::process::Command;
use wait_utils::WaitStrategy;

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
        wait_strategy: &WaitStrategy,
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
        self.wait_for_program(wait_strategy)
            .await
            .expect("Failed to wait for program");

        self.dbg_print("Service started");
        Ok(())
    }

    /// Waits for the program to become ready based on the given wait strategy.
    ///
    /// # Arguments
    ///
    /// * `wait_strategy` - The strategy used to determine when the program is ready.
    ///
    /// # Returns
    ///
    /// Returns a `ServiceUtilError` if waiting for the program fails.
    ///
    /// # Panics
    ///
    /// Panics if the `WaitStrategy::WaitUntilConsoleOutputContains` is used,
    /// as it is not supported.
    ///
    pub(crate) async fn wait_for_program(
        &self,
        wait_strategy: &WaitStrategy,
    ) -> Result<(), ServiceUtilError> {
        match wait_strategy {
            WaitStrategy::WaitForDuration(duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {duration} seconds."
                ));
                wait_utils::wait_until_timeout(duration).expect("Failed to wait for duration");
            }

            WaitStrategy::WaitUntilConsoleOutputContains(_, _) => {
                panic!("WaitUntilConsoleOutputContains is not supported!");
            }

            WaitStrategy::WaitForHttpHealthCheck(url, duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {:?} on HTTP health check on {}.",
                    duration, url
                ));
                wait_utils::wait_until_http_health_check(self.dbg, url, duration)
                    .expect("Failed to wait for HTTP health check");
            }

            WaitStrategy::WaitForGrpcHealthCheck(url, duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {:?} on GRPC health check on {}.",
                    duration, url
                ));
                wait_utils::wait_until_grpc_health_check(self.dbg, url, duration)
                    .expect("Failed to wait for HTTP health check");
            }

            WaitStrategy::NoWait => {
                self.dbg_print("[start_container]: No wait. Return immediately.");
                // Do nothing
            }
        };
        Ok(())
    }
}
