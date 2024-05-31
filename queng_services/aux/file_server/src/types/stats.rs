use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    number_assets: u32,
    number_exchanges: u32,
    number_instruments: u32,
    number_unique_symbols: u32,
}

impl Stats {
    pub fn new(
        number_assets: u32,
        number_exchanges: u32,
        number_instruments: u32,
        number_unique_symbols: u32,
    ) -> Self {
        Self {
            number_assets,
            number_exchanges,
            number_instruments,
            number_unique_symbols,
        }
    }
}
