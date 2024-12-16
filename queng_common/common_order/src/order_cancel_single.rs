use common_exchange::ExchangeID;
use std::fmt;
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OrderCancelSingle {
    exchange_id: ExchangeID,
    client_id: u16,
    symbol_id: String,
    client_order_id: String,
    exchange_order_id: String,
}

impl OrderCancelSingle {
    pub fn new(
        exchange_id: ExchangeID,
        client_id: u16,
        symbol_id: String,
        client_order_id: String,
        exchange_order_id: String,
    ) -> Self {
        Self {
            exchange_id,
            client_id,
            symbol_id,
            client_order_id,
            exchange_order_id,
        }
    }
}

impl OrderCancelSingle {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }

    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }

    pub fn exchange_order_id(&self) -> &str {
        &self.exchange_order_id
    }
}

impl Display for OrderCancelSingle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
