use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderFill {
    // Execution time in UTC
    time: DateTime<Utc>,
    // Execution time in Unix timestamp
    timestamp: u64,
    // Execution price
    price: Decimal,
    // Execution quantity
    quantity: Decimal,
}

impl OrderFill {
    pub fn new(time: DateTime<Utc>, timestamp: u64, price: Decimal, quantity: Decimal) -> Self {
        Self {
            time,
            timestamp,
            price,
            quantity,
        }
    }
}

impl OrderFill {
    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl Display for OrderFill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
