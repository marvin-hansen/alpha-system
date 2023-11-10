use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// A ServiceType represents the type of service.
///
/// # Variants
///
/// * `ENDPOINT`: An endpoint service type.
/// * `CHANNEL`: The channel service type.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum ServiceType {
    /// The endpoint service type.
    #[default]
    ENDPOINT,
    /// The channel service type.
    CHANNEL,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::ENDPOINT => write!(f, "ENDPOINT"),
            ServiceType::CHANNEL => write!(f, "CHANNEL"),
        }
    }
}
