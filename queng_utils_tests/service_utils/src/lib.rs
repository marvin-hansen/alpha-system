use std::fmt::{Display, Formatter};

use ctx_manager::CtxManager;

use crate::error::service_util_error::ServiceUtilError;

mod error;
mod fields;
pub mod prelude;
mod service;
mod types;
mod verify;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ServiceUtil {
    dbg: bool,
    ctx_manager: CtxManager,
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

        let ctx_manager = if dbg {
            CtxManager::with_debug().await
        } else {
            CtxManager::new().await
        };

        let env = ctx_manager.env_type();

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

        Ok(Self { dbg, ctx_manager })
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
