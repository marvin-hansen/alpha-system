use common_exchange::ExchangeID;
use std::fmt;
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OrderCancelSingle {
    exchange_id: ExchangeID,
    symbol_id: String,
}

impl OrderCancelSingle {
    pub fn new(exchange_id: ExchangeID, symbol_id: String) -> Self {
        Self {
            exchange_id,
            symbol_id,
        }
    }
}

impl OrderCancelSingle {
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }
}

impl Display for OrderCancelSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
