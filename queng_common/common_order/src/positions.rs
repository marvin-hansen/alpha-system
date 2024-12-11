use crate::position_data::PositionData;
use common_exchange::ExchangeID;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Positions {
    exchange_id: ExchangeID,
    positions: Vec<PositionData>,
}

impl Positions {
    pub fn new(exchange_id: ExchangeID, positions: Vec<PositionData>) -> Self {
        Self {
            exchange_id,
            positions,
        }
    }
}

impl Positions {
    pub fn exchange_id(&self) -> &ExchangeID {
        &self.exchange_id
    }

    pub fn positions(&self) -> &Vec<PositionData> {
        &self.positions
    }
}

impl Display for Positions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
