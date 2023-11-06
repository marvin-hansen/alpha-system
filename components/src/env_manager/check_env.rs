use std::env;

use common::prelude::EnvironmentType;

pub fn get_env_type() -> EnvironmentType {
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
