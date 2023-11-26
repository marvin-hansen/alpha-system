use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum AccountType {
    UnknownAccountType,
    #[default]
    Spot,
    Margin,
    Future,
}

impl Display for AccountType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::UnknownAccountType => write!(f, "UnknownAccountType"),
            AccountType::Spot => write!(f, "Spot"),
            AccountType::Margin => write!(f, "Margin"),
            AccountType::Future => write!(f, "Future"),
        }
    }
}
