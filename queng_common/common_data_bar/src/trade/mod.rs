use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct Trade {
    symbol: String,
    date_time: DateTime<Utc>,
    price: Decimal,
    quantity: Decimal,
}

impl Trade {
    pub fn new(
        symbol: String,
        date_time: DateTime<Utc>,
        price: Decimal,
        quantity: Decimal,
    ) -> Self {
        Self {
            symbol,
            date_time,
            price,
            quantity,
        }
    }
}

impl Trade {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl Display for Trade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Trade {{ symbol: {}, date_time: {}, price: {}, quantity: {} }}",
            self.symbol, self.date_time, self.price, self.quantity
        )
    }
}
