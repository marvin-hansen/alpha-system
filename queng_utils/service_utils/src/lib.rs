use crate::error::service_util_error::ServiceUtilError;
use config_manager::CfgManager;
use std::fmt::{Display, Formatter};
use std::time::Duration;

mod error;
mod fields;
pub mod prelude;
mod service;
mod types;
mod verify;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceWaitStrategy {
    Duration(Duration),
    HttpHealthCheck(String, Duration),
}

#[derive(Debug)]
pub struct ServiceUtil {
    dbg: bool,
    config_manager: CfgManager,
}

impl ServiceUtil {
    /// Creates a new `ServiceUtil` instance with debug mode disabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ServiceUtil` instance on success.
    /// On failure, returns a `ServiceUtilError`.
    ///
    pub async fn new() -> Result<Self, ServiceUtilError> {
        Self::build(false).await
    }

    /// Creates a new `ServiceUtil` instance with debug mode enabled.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ServiceUtil` instance on success.
    /// On failure, returns a `ServiceUtilError`.
    ///
    pub async fn with_debug() -> Result<Self, ServiceUtilError> {
        Self::build(true).await
    }

    async fn build(dbg: bool) -> Result<Self, ServiceUtilError> {
        if dbg {
            println!("[ServiceUtil]: Debug mode enabled");
        }

        let config_manager = if dbg {
            CfgManager::default_with_debug()
        } else {
            CfgManager::default()
        };

        let env = config_manager.env_type();

        if dbg {
            println!(
                "[ServiceUtil]: Verify all binaries for environment: {:?}",
                env
            );
        }
        match verify::verify_binary::verify_all_binaries(dbg, env) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to verify binaries: {}", e)
            }
        }

        Ok(Self {
            dbg,
            config_manager,
        })
    }

    pub fn config_manager(&self) -> &CfgManager {
        &self.config_manager
    }
}

impl ServiceUtil {
    pub fn dbg_print(&self, s: &str) {
        if self.dbg {
            println!("[ServiceUtil]: {}", s);
        }
    }
}

impl Display for ServiceUtil {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceUtil {{ debug mode: {} }}", &self.dbg)
    }
}
