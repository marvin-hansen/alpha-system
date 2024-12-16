use common_exchange::ExchangeID;
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OrderCancelAll {
    client_id: u16,
    exchange_id: ExchangeID,
}

impl OrderCancelAll {
    pub fn new(client_id: u16, exchange_id: ExchangeID) -> Self {
        Self {
            client_id,
            exchange_id,
        }
    }
}

impl OrderCancelAll {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }

    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }
}

impl Display for OrderCancelAll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
