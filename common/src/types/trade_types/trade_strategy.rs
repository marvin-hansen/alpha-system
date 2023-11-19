use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub enum TradeStrategyType {
    #[default]
    BuyHold,
    TurboTrend,
    CausalBreakout,
}

impl Display for TradeStrategyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeStrategyType::BuyHold => write!(f, "BuyHold"),
            TradeStrategyType::TurboTrend => write!(f, "TurboTrend"),
            TradeStrategyType::CausalBreakout => write!(f, "CausalBreakout"),
        }
    }
}
