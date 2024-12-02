use crate::{DockerError, DockerUtil};
use common_container::WaitStrategy;
use std::process::Command;
use std::time::{Duration, Instant};
impl DockerUtil {
    /// Waits for a new Docker container to finish starting.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container.
    /// * `wait_strategy` - The wait strategy to use for the container.
    ///
    /// # Returns
    ///
    /// Returns Ok if successful,
    /// or a `DockerError` if an error occurs.
    pub(crate) fn wait_for_container(
        &self,
        container_id: &str,
        wait_strategy: &WaitStrategy,
    ) -> Result<(), DockerError> {
        match wait_strategy {
            WaitStrategy::WaitForDuration(duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {duration} seconds."
                ));
                std::thread::sleep(Duration::from_secs(*duration));
                Ok(())
            }
            WaitStrategy::WaitUntilConsoleOutputContains(expected_output, timeout) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting until console output contains '{expected_output}'"
                ));
                self.wait_until_console_output_contains(container_id, expected_output, timeout)
                    .expect("Failed to wait until console output contains");

                Ok(())
            }
            WaitStrategy::WaitForHttpHealthCheck(url, duration) => {
                self.dbg_print(&format!(
                    "[start_container]: Waiting for {:?} on HTTP health check on {}.",
                    duration, url
                ));
                self.wait_for_http_health_check(url, duration)
                    .expect("Failed to wait for HTTP health check");

                Ok(())
            }

            WaitStrategy::NoWait => {
                self.dbg_print("[start_container]: No wait. Return immediately.");
                // Do nothing
                Ok(())
            }
        }
    }
}

impl DockerUtil {
    /// Waits until the console output of the container with the given ID contains the
    /// specified expected output. If the expected output is not found within the given
    /// timeout, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The ID of the container whose console output to check.
    /// * `expected_output` - The string to search for in the console output.
    /// * `timeout` - The timeout duration in seconds.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the expected output is found within the timeout, or an
    /// `Err(DockerError)` if the expected output is not found.
    ///
    pub(crate) fn wait_until_console_output_contains(
        &self,
        container_id: &str,
        expected_output: &str,
        timeout: &u64,
    ) -> Result<(), DockerError> {
        let start_time = Instant::now();
        let timeout = Duration::from_secs(*timeout);
        self.dbg_print("wait_until_console_output_contains");

        loop {
            std::thread::sleep(Duration::from_millis(100));

            if start_time.elapsed() > timeout {
                return Err(DockerError::from(format!(
                    "[start_container]: !!Timeout!! Waited {} seconds for console output to contain {}",
                    timeout.as_secs(),
                    expected_output
                )));
            }

            // Example: docker logs apiproxy-7777
            // https://docs.docker.com/reference/cli/docker/container/logs/
            let output = match Command::new("docker")
                .arg("logs")
                .arg(container_id)
                .output()
                .map_err(|e| {
                    DockerError::from(format!(
                        "[start_container]: Failed to run docker logs for container: {container_id} Error: {e}"
                    ))
                }) {
                Ok(o) => o,
                Err(e) => return Err(e),
            };

            if output.status.success() {
                self.dbg_print("status.success");
                if String::from_utf8_lossy(&output.stdout).contains(expected_output) {
                    self.dbg_print("MATCH: stdout contains expected output");

                    // Apparently, when the success log message appears in Docker,
                    // some services still need more time to become ready.
                    std::thread::sleep(Duration::from_millis(250));
                    break;
                }
            }
        }

        if self.dbg {
            // construct docker docker ps -a
            let mut cmd = Command::new("docker");
            cmd.arg("logs").arg("-t").arg(container_id);
            let output = cmd.output().expect("Failed to run docker logs");
            println!(
                "DEBUG: container logs: {}",
                String::from_utf8_lossy(&output.stdout)
            );
        }

        Ok(())
    }

    /// Waits for a HTTP health check to succeed.
    ///
    /// # Arguments
    ///
    /// * `health_url` - The URL to check for health status.
    /// * `timeout` - The maximum duration to wait for a successful health check.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the health check is successful within the timeout period,
    /// or a `DockerError` if it fails or times out.
    ///
    pub(crate) fn wait_for_http_health_check(
        &self,
        health_url: &str,
        timeout: &Duration,
    ) -> Result<(), DockerError> {
        let start_time = Instant::now();

        loop {
            std::thread::sleep(Duration::from_millis(100));

            if start_time.elapsed().as_secs() > timeout.as_secs() {
                return Err(DockerError(format!(
                    "[wait_until_http_health_check]: !!Timeout!! Waited {} seconds for service to respond to health check",
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
