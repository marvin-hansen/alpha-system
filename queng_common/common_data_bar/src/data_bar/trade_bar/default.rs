use crate::prelude::TradeBar;
use chrono::Utc;
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;

impl Default for TradeBar {
    fn default() -> Self {
        Self {
            symbol_id: 1,
            date_time: Utc::now(),
            price: Decimal::zero(),
            volume: Decimal::zero(),
        }
    }
}
