use rust_decimal::Decimal;
use std::fmt;
use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataBar {
    date_time: String,// Cannot serialize DataTime with serde hence String
    symbol: String,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume: Decimal,
    trades: Decimal,
}

impl DataBar {
    pub fn new(date_time: String, symbol: String, open: Decimal, high: Decimal, low: Decimal, close: Decimal, volume: Decimal, trades: Decimal) -> Self {
        Self { date_time, symbol, open, high, low, close, volume, trades }
    }
}

impl DataBar {

    pub fn date_time(&self) -> String {
        self.date_time.clone()
    }

    pub fn symbol(&self) -> String {
        self.symbol.to_string()
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

    pub fn trades(&self) -> Decimal {
        self.trades
    }
}

impl Display for DataBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DataTime: {},\n Symbol {},\n Open {},\n High {},\n Low {},\n Close {},\n Volume {},\n Trades {}",
               self.date_time, self.symbol, self.open, self.high, self.low, self.close, self.volume, self.trades)
    }
}
