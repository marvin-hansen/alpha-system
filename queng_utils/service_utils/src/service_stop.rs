use std::process::Command;
use std::time::Duration;

use common_config::prelude::ServiceID;

use crate::error::ServiceUtilError;
use crate::ServiceUtil;

impl ServiceUtil {
    /// Stops a running service by its ID.
    ///
    /// # Arguments
    ///
    /// * `svc` - The ID of the service to stop.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the service was successfully stopped, or
    /// `Err(ServiceUtilError)` if an error occurred.
    ///
    pub fn stop_service(&self, svc: &ServiceID) -> Result<(), ServiceUtilError> {
        self.dbg_print("stop_service");
        self.dbg_print(&format!("Stopping service {}...", svc));

        {
            let map = self.services.borrow();
            if !map.contains_key(svc) {
                self.dbg_print(&format!("Service {} not running", svc));
                return Err(ServiceUtilError::ServiceNotRunning(svc.to_string()));
            }
        }

        let pid = {
            let map = self.services.borrow();
            map.get(svc).unwrap().to_string()
        };

        self.dbg_print(format!("Stopping PID {}", &pid).as_str());

        self.dbg_print("Constructing stop command");
        let mut cmd = Command::new("kill");
        cmd.arg(pid);

        self.dbg_print(&format!("Run command: {:?}", &cmd));

        match cmd.output() {
            Ok(out) => {
                if !out.status.success() {
                    self.dbg_print(&format!("Error stopping service: {}", svc));
                    self.dbg_print(&format!(
                        "service: {}
                         success: {}
                         Output: {}",
                        svc.to_string(),
                        out.status.success(),
                        String::from_utf8_lossy(out.stdout.as_slice()),
                    ));

                    return Err(ServiceUtilError::ServiceStopFailed(format!(
                        "Error stopping service {}: {}",
                        svc,
                        String::from_utf8_lossy(&out.stdout).trim()
                    )));
                }
            }
            Err(e) => {
                return Err(ServiceUtilError::ServiceStopFailed(format!(
                    "Error stopping service {}: {}",
                    svc, e
                )))
            }
        };
        // There is a minimal delay between sending kill and the processes having
        // completely stopped. Let's wait a moment for that.
        std::thread::sleep(Duration::from_millis(100));

        self.dbg_print("Removing service from map");
        {
            let mut map = self.services.borrow_mut();
            map.remove(svc);
        }

        self.dbg_print("Success, service stopped");
        Ok(())
    }
}
