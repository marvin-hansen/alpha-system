use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum ExchangeID {
    UnknownExchange,
    Binance,
    #[default]
    VEX,
}

impl Display for ExchangeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExchangeID::UnknownExchange => write!(f, "UnknownExchange"),
            ExchangeID::Binance => write!(f, "Binance"),
            ExchangeID::VEX => write!(f, "VEX"),
        }
    }
}
