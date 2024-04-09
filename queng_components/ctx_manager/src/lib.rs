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
    int_dns_server: Option<String>,
}

impl CtxManager {
    /// Creates a new CtxManager instance.
    pub fn new() -> Self {
        let env_type = get_env_type();

        let int_dns_server =
            if env_type == EnvironmentType::CLUSTER || env_type == EnvironmentType::CI {
                get_int_cluster_dns_server()
            } else {
                None
            };

        Self {
            env_type,
            int_dns_server,
        }
    }
}

impl CtxManager {
    /// Returns the environment type.
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
    /// Returns the internal DNS server.
    pub fn int_dns_server(&self) -> &Option<String> {
        &self.int_dns_server
    }
}

impl Display for CtxManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CtxManager {{ env_type: {:?}, int_dns_server: {:?} }}",
            self.env_type, self.int_dns_server
        )
    }
}

// Check if the environment variable is set.
// If not, check if an .env file is present.
// If so, return local environment as the file only exists locally.
// If not, return UnknownEnv.
// Note, on Mac OS, shell environment variables are sanitized (erased) by default for security reasons
// thus the presence of an .env file is used to identify a local environment.
fn get_env_type() -> EnvironmentType {
    return match env::var("ENV") {
        Ok(val) => match val.as_str() {
            "CI" => EnvironmentType::CI,
            "CLUSTER" => EnvironmentType::CLUSTER,
            "LOCAL" => EnvironmentType::LOCAL,
            "UNKNOWN" => EnvironmentType::UnknownEnv,
            _ => EnvironmentType::UnknownEnv,
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("Failed to read ENV environment variable. Ensure ENV is set");
        }
    };
}

fn get_int_cluster_dns_server() -> Option<String> {
    // Check if the environment variable is set. If not, return the default value UnknownEnv.
    let dns_server_var = match env::var("DNS_SERVER") {
        Ok(val) => val,
        Err(e) => {
            panic!(
                "Failed to read DNS_SERVER environment variable. Ensure DNS_SERVER is set in deployment.yaml:{}",
                e
            );
        }
    };
    Some(dns_server_var)
}
