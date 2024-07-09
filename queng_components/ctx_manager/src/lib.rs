use std::env;
use std::fmt::{Display, Formatter};

use common::prelude::EnvironmentType;

/// The CtxManager struct manages the context information for the application,
/// such as the environment type and the internal DNS server. To work in a cluster context,
/// ensure that the following environment variables are set:
///         ENV: CLUSTER
///         DNS_SERVER: 175.24.54.1 //  IP of your actual cluster DNS server
///
/// PANICS if either one of the variables is not set.
///
/// # Fields
/// * `env_type`: The environment type, which can be either `LOCAL`, `CLUSTER`, or `UNKNOWN`.
/// * `int_dns_server`: The internal DNS server.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CtxManager {
    env_type: EnvironmentType,
}

impl CtxManager {
    /// Creates a new CtxManager instance.
    pub fn new() -> Self {
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

        Self { env_type }
    }
}

impl CtxManager {
    /// Returns the environment type.
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }

    pub fn env_var(&self) -> &str {
        match self.env_type {
            EnvironmentType::UNKNOWN => "ENV=UNKNOWN",
            EnvironmentType::LOCAL => "ENV=LOCAL",
            EnvironmentType::CLUSTER => "ENV=CLUSTER",
            EnvironmentType::CI => "ENV=CI",
        }
    }
}

impl Display for CtxManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CtxManager {{ env_type: {:?} }}", &self.env_type)
    }
}
