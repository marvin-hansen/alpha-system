use common_exchange::ExchangeID;
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OrderCancelAll {
    exchange_id: ExchangeID,
}

impl OrderCancelAll {
    pub fn new(exchange_id: ExchangeID) -> Self {
        Self { exchange_id }
    }
}

impl From<ExchangeID> for OrderCancelAll {
    fn from(exchange_id: ExchangeID) -> Self {
        Self { exchange_id }
    }
}

impl From<OrderCancelAll> for ExchangeID {
    fn from(order_cancel_single: OrderCancelAll) -> Self {
        order_cancel_single.exchange_id
    }
}

impl OrderCancelAll {
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }
}

impl Display for OrderCancelAll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
