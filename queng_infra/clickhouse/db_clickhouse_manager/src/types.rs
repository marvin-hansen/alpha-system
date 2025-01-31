/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use chrono::{DateTime, TimeZone, Utc};
use klickhouse::{DateTime64, Row};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct TradeRow {
    date_time: DateTime64<3>,
    price: f64,
    volume: f64,
}

impl TradeRow {
    #[must_use]
    pub fn date_time(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.date_time.1 as i64).unwrap()
    }
    #[must_use]
    pub fn price(&self) -> Decimal {
        Decimal::from_f64(self.price).unwrap()
    }
    #[must_use]
    pub fn volume(&self) -> Decimal {
        Decimal::from_f64(self.volume).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct OHLCVRow {
    datetime: u32,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl OHLCVRow {
    #[must_use]
    pub fn date_time(&self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(i64::from(self.datetime)).unwrap()
    }
    #[must_use]
    pub fn open(&self) -> Decimal {
        Decimal::from_f64(self.open).unwrap()
    }
    #[must_use]
    pub fn high(&self) -> Decimal {
        Decimal::from_f64(self.high).unwrap()
    }
    #[must_use]
    pub fn low(&self) -> Decimal {
        Decimal::from_f64(self.low).unwrap()
    }
    #[must_use]
    pub fn close(&self) -> Decimal {
        Decimal::from_f64(self.close).unwrap()
    }
    #[must_use]
    pub fn volume(&self) -> Decimal {
        Decimal::from_f64(self.volume).unwrap()
    }
}

#[derive(Debug, Eq, Clone, PartialEq, Row, Serialize, Deserialize)]
pub struct SymbolRow {
    symbol_id: u64,
    symbol: String,
}

impl SymbolRow {
    #[must_use]
    pub const fn symbol_id(&self) -> u64 {
        self.symbol_id
    }
    #[must_use]
    pub fn symbol(&self) -> String {
        self.symbol.to_string()
    }
}
