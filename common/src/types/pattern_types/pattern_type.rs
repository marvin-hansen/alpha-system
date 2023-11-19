use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum PatternType {
    #[default]
    Base,
    Extra,
}

impl Display for PatternType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternType::Base => write!(f, "Base"),
            PatternType::Extra => write!(f, "Extra"),
        }
    }
}
