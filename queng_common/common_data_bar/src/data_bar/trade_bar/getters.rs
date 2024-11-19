use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::TradeBar;

impl TradeBar {
    #[must_use]
    pub const fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }
    #[must_use]
    pub const fn price(&self) -> Decimal {
        self.price
    }
    #[must_use]
    pub const fn volume(&self) -> Decimal {
        self.volume
    }
    #[must_use]
    pub const fn symbol_id(&self) -> u16 {
        self.symbol_id
    }
}
