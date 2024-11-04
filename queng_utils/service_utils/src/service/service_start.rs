use common_config::prelude::ServiceID;
use std::process::Command;
use std::time::Duration;
use tokio::time::sleep;

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
    pub async fn start_service(
        &self,
        svc: &ServiceID,
        wait_duration: Duration,
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
        sleep(wait_duration).await;

        self.dbg_print("Service started");

        Ok(())
    }
}
