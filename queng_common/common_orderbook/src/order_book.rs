use crate::{Ask, Bid};
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Default, Eq, Clone, PartialEq)]
pub struct Orderbook {
    symbol: String,
    time_exchange: DateTime<Utc>,
    time_integration: DateTime<Utc>,
    asks: Arc<Vec<Ask>>,
    bids: Arc<Vec<Bid>>,
}

impl Orderbook {
    pub fn new(
        symbol: String,
        time_exchange: DateTime<Utc>,
        time_integration: DateTime<Utc>,
        asks: Vec<Ask>,
        bids: Vec<Bid>,
    ) -> Self {
        Self {
            symbol,
            time_exchange,
            time_integration,
            asks: Arc::new(asks),
            bids: Arc::new(bids),
        }
    }
}

impl Orderbook {
    pub fn asks_first_n(&self, n: usize) -> Vec<&Ask> {
        self.asks.iter().take(n).collect()
    }

    pub fn bids_first_n(&self, n: usize) -> Vec<&Bid> {
        self.bids.iter().take(n).collect()
    }
}

impl Orderbook {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn time_exchange(&self) -> DateTime<Utc> {
        self.time_exchange
    }

    pub fn time_integration(&self) -> DateTime<Utc> {
        self.time_integration
    }

    pub fn asks(&self) -> Vec<&Ask> {
        self.asks.iter().collect()
    }

    pub fn bids(&self) -> Vec<&Bid> {
        self.bids.iter().collect()
    }
}

impl Display for Orderbook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
