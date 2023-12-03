use std::fmt::{Display, Formatter};

/// An EnvironmentType represents the environment type of the application.
///
/// # Variants
///
/// * `UnknownEnv`: The unknown environment type.
/// * `LOCAL`: The local environment type.
/// * `CLUSTER`: The cluster environment type.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum EnvironmentType {
    /// The unknown environment type.
    #[default]
    UnknownEnv,
    /// The local environment type.
    LOCAL,
    /// The cluster environment type.
    CLUSTER,
    /// Continuous Integration (CI) environment type.
    CI,
    /// Docker environment type here.
    Docker,
}

impl Display for EnvironmentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentType::UnknownEnv => write!(f, "UnknownEnv"),
            EnvironmentType::LOCAL => write!(f, "LOCAL"),
            EnvironmentType::CLUSTER => write!(f, "CLUSTER"),
            EnvironmentType::CI => write!(f, "CI"),
            EnvironmentType::Docker => write!(f, "Docker"),
        }
    }
}
