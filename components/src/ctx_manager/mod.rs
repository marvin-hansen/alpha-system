use std::env;
use std::fmt::{Display, Formatter};

use common::prelude::EnvironmentType;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct CtxManager {
    env_type: EnvironmentType,
    int_dns_server: Option<String>,
}

impl CtxManager {
    pub fn new() -> Self {
        let env_type = get_env_type();
        let int_dns_server = get_int_dns_server();
        Self {
            env_type,
            int_dns_server,
        }
    }
}

impl CtxManager {
    pub fn env_type(&self) -> EnvironmentType {
        self.env_type
    }
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

fn get_env_type() -> EnvironmentType {
    // Check if the environment variable is set. If not, return the default value UnknownEnv.
    let env_var = match env::var("ENV") {
        Ok(val) => val,
        Err(_) => return EnvironmentType::UnknownEnv,
    };

    // Convert the environment variable to an EnvironmentType enum value.
    return match env_var.as_str() {
        "LOCAL" => EnvironmentType::LOCAL,
        "CLUSTER" => EnvironmentType::CLUSTER,
        "UNKNOWN" => EnvironmentType::UnknownEnv,
        _ => EnvironmentType::UnknownEnv,
    };
}

fn get_int_dns_server() -> Option<String> {
    // Check if the environment variable is set. If not, return the default value UnknownEnv.
    let dns_server_var = match env::var("DNS_SERVER") {
        Ok(val) => val,
        Err(e) => {
            println!(
                "Failed to read DNS_SERVER env. Ensure DNS_SERVER is set in deployment.yaml:m {}",
                e
            );
            return None;
        }
    };

    Some(dns_server_var)
}
