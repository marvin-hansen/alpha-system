use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TradeDirection {
    Buy,
    Sell,
    Hold,
}

impl Display for TradeDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "Buy"),
            Self::Sell => write!(f, "Sell"),
            Self::Hold => write!(f, "Hold"),
        }
    }
}
