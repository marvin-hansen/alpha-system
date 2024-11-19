use crate::OHLCVBar;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

impl OHLCVBar {
    #[must_use]
    pub fn range_change(&self) -> Decimal {
        self.close - self.open
    }

    #[must_use]
    pub fn range_percent(&self) -> Decimal {
        let one_hundred = Decimal::new(100, 0);
        (((self.close - self.open) / self.open) * one_hundred).round_dp(4)
    }
}

impl OHLCVBar {
    #[must_use]
    pub const fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    #[must_use]
    pub const fn open(&self) -> Decimal {
        self.open
    }

    #[must_use]
    pub const fn high(&self) -> Decimal {
        self.high
    }

    #[must_use]
    pub const fn low(&self) -> Decimal {
        self.low
    }

    #[must_use]
    pub const fn close(&self) -> Decimal {
        self.close
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
