use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum TradeStrategyType {
    #[default]
    BuyHold,
    TurboTrend,
    CausalBreakout,
}

impl Display for TradeStrategyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuyHold => write!(f, "BuyHold"),
            Self::TurboTrend => write!(f, "TurboTrend"),
            Self::CausalBreakout => write!(f, "CausalBreakout"),
        }
    }
}
