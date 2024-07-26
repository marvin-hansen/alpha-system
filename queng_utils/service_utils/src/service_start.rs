use std::process::Command;

use common_config::prelude::ServiceID;

use crate::error::ServiceUtilError;
use crate::prelude::ServiceStartFailed;
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
    pub fn start_service(&self, svc: &ServiceID) -> Result<(), ServiceUtilError> {
        self.dbg_print("start_service");
        self.dbg_print(&format!("Starting service: {}", svc));

        let program = match self.get_service_target(svc) {
            Ok(target) => target,
            Err(e) => {
                self.dbg_print(&format!("No run target for service {svc} found"));
                return Err(e);
            }
        };

        self.dbg_print("Constructing initial command");
        let mut cmd = Command::new(program);

        self.dbg_print("Setting environment variables");
        let (env, val) = self.ctx_manager.env_var();
        self.dbg_print(format!("{env}={val}").as_str());
        cmd.env(env, val);
        cmd.arg("&");

        self.dbg_print(&format!("Run command: {:?}", &cmd));

        let pid = match cmd.output() {
            Ok(out) => {
                self.dbg_print(&format!(
                    "service: {}
                        success: {}
                        Output: {}",
                    svc,
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                ));

                // In Case of error, print the output
                if !out.status.success() {
                    return Err(ServiceStartFailed(format!(
                        "Error starting service {}: {}",
                        svc,
                        String::from_utf8_lossy(&out.stdout).trim()
                    )));
                }

                let pid = String::from_utf8_lossy(&out.stdout)
                    .trim()
                    .replace("[1] ", "");
                self.dbg_print(&format!("PID: {}", pid));
                pid
            }
            Err(e) => {
                return Err(ServiceStartFailed(format!(
                    "Error starting service {}: {}",
                    svc, e
                )))
            }
        };

        // Figure out how to wait until the service is known to be running

        // Insert the PID into the service map
        {
            let mut map = self.services.borrow_mut();
            map.insert(*svc, pid);
        }

        Ok(())
    }
}
