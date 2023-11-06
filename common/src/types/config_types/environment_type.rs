use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum EnvironmentType {
    #[default]
    UnknownEnv,
    LOCAL,
    CLUSTER,
}

impl Display for EnvironmentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvironmentType::UnknownEnv => write!(f, "UnknownEnv"),
            EnvironmentType::LOCAL => write!(f, "LOCAL"),
            EnvironmentType::CLUSTER => write!(f, "CLUSTER"),
        }
    }
}
