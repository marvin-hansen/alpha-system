use rust_decimal::Decimal;
use std::fmt;
use std::fmt::{Debug, Display};
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use crate::prelude::SymbolID;


#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBar {
    date_time: DateTime<Utc>,
    symbol: SymbolID,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
}

impl DataBar {
    pub fn new(
        date_time: DateTime<Utc>,
        symbol: SymbolID,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            date_time,
            symbol,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

impl Default for DataBar
{
    fn default() -> Self
    {
        Self
        {
            date_time: Utc::now(),
            symbol: SymbolID::default(),
            open: Decimal::default(),
            high: Decimal::default(),
            low: Decimal::default(),
            close: Decimal::default(),
            volume: Decimal::default(),
        }
    }
}


impl DataBar {

    pub fn range_change(&self) -> Decimal {
        self.close - self.open
    }

    pub fn range_percent(&self) -> Decimal
    {
        let one_hundred = Decimal::new(100, 0);
        (((self.close - self.open) / self.open) * one_hundred).round_dp(4)
    }

}

impl DataBar {
    pub fn date_time(&self) -> DateTime<Utc> {
        self.date_time
    }

    pub fn symbol(&self) -> SymbolID {
        self.symbol
    }

    pub fn open(&self) -> Decimal {
        self.open
    }

    pub fn high(&self) -> Decimal {
        self.high
    }

    pub fn low(&self) -> Decimal {
        self.low
    }

    pub fn close(&self) -> Decimal {
        self.close
    }

    pub fn volume(&self) -> Decimal {
        self.volume
    }

}

impl Display for DataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataTime: {},\n Symbol {},\n Open {},\n High {},\n Low {},\n Close {},\n Volume {}",
               self.date_time, self.symbol, self.open, self.high, self.low, self.close, self.volume)
    }
}
