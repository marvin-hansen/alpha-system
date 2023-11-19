use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum TradeDirection {
    Buy,
    Sell,
    Hold,
}

impl Display for TradeDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeDirection::Buy => write!(f, "Buy"),
            TradeDirection::Sell => write!(f, "Sell"),
            TradeDirection::Hold => write!(f, "Hold"),
        }
    }
}
