use std::collections::HashMap;
use common::prelude::DataBar;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BarManager {
    bars: HashMap<String, Vec<DataBar>>,
}

impl BarManager {
    /// Creates a new instance of `BarManager`.
    pub fn new() -> Self {
        Self {
            bars: HashMap::new(),
        }
    }
}

impl BarManager {
    pub fn add_bars(&mut self, symbol: &str, bars: Vec<DataBar>) {

        self.bars.insert(symbol.to_string(), bars);
    }

    pub fn remove_bars(&mut self, symbol: &str) {
        self.bars.remove(symbol);
    }

    pub fn has_data(&self, symbol: &str) -> bool {
        self.bars.contains_key(symbol)
    }

    pub fn get_bars(&self, symbol: &str) -> Result<Vec<DataBar>, &'static str> {
        match self.bars.get(symbol) {
            Some(bars) => Ok(bars.to_vec()),
            None => Err("No data found for symbol")
        }
    }
}

