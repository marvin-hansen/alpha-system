use std::env;
use std::fmt::{Display, Formatter};

use common_env::prelude::EnvironmentType;

/// The CtxManager struct manages the context information for the application,
/// such as the environment type and the internal DNS server. To work in a cluster context,
/// ensure that the following environment variables are set:
///         ENV: CLUSTER
///
/// PANICS if the variables is not set.
///
/// # Fields
/// * `env_type`: The environment type, which can be either `LOCAL`, `CI`, `CLUSTER`, or `UNKNOWN`.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CtxManager {
    dbg: bool,
    env_type: EnvironmentType,
}

impl CtxManager {
    /// Creates a new CtxManager instance.
    pub fn new() -> Self {
        Self::build(false)
    }

    pub fn with_debug() -> Self {
        Self::build(true)
    }

    fn build(dbg: bool) -> Self {
        if dbg {
            println!("[CtxManager]: Debug mode enabled");
        }

        // Check if the environment variable is set.
        // If so, return local environment as the file only exists locally.
        // If not, return UnknownEnv.
        // On Mac OS, each shell environment variables is sanitized (erased) by default for security reasonsm
        let env_type = match env::var("ENV") {
            Ok(val) => match val.as_str() {
                "CI" => EnvironmentType::CI,
                "CLUSTER" => EnvironmentType::CLUSTER,
                "LOCAL" => EnvironmentType::LOCAL,
                "UNKNOWN" => EnvironmentType::UNKNOWN,
                _ => EnvironmentType::UNKNOWN,
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!("Failed to read ENV environment variable. Ensure ENV is set");
            }
        };

        if dbg {
            println!("[CtxManager]: Detected environment type: {:?}", &env_type);
        }

        Self { dbg, env_type }
    }
}

impl CtxManager {
    /// Returns the environment type.
    pub fn env_type(&self) -> EnvironmentType {
        self.dbg_print("env_type");
        self.env_type
    }

    pub fn env_var(&self) -> (String, String) {
        match self.env_type {
            EnvironmentType::UNKNOWN => ("ENV".to_string(), "UNKNOWN".to_string()),
            EnvironmentType::LOCAL => ("ENV".to_string(), "LOCAL".to_string()),
            EnvironmentType::CLUSTER => ("ENV".to_string(), "CLUSTER".to_string()),
            EnvironmentType::CI => ("ENV".to_string(), "CI".to_string()),
        }
    }
}

impl CtxManager {
    pub fn dbg_print(&self, msg: &str) {
        if self.dbg {
            println!("[CtxManager]: {}", msg);
        }
    }
}

impl Display for CtxManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CtxManager {{ env_type: {:?} }}", &self.env_type)
    }
}
