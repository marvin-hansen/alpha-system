use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum TradeEntryType {
    #[default]
    CurrentBar,
    NextBar,
}

impl Display for TradeEntryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeEntryType::CurrentBar => write!(f, "CurrentBar"),
            TradeEntryType::NextBar => write!(f, "NextBar"),
        }
    }
}
