use common::prelude::{DataBar, FileConfigType};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BarManager {
    bars: HashMap<FileConfigType, Vec<DataBar>>,
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
    /// Adds a vector of DataBar for the given symbol
    ///
    /// # Parameters
    ///
    /// * `symbol` - Symbol name as a &str
    /// * `bars` - Vector of DataBar to add
    ///
    /// # Remarks
    ///
    /// This will insert the bars into the internal map, associating
    /// the symbol with the bar data.
    ///
    pub fn add_bars(&mut self, symbol: &FileConfigType, bars: Vec<DataBar>) {
        self.bars.insert(*symbol, bars);
    }

    /// Removes the bars for the given symbol
    ///
    /// # Parameters
    ///
    /// * `symbol` - Symbol name as a &str
    ///
    /// # Remarks
    ///
    /// This will remove the symbol and associated bars from the
    /// internal map if present.
    ///
    pub fn remove_bars(&mut self, symbol: &FileConfigType) {
        self.bars.remove(symbol);
    }

    /// Checks if data is available for the given symbol
    ///
    /// # Parameters
    ///
    /// * `symbol` - Symbol to check as a &str
    ///
    /// # Returns
    ///
    /// bool - true if data is available, false otherwise
    pub fn has_data(&self, symbol: &FileConfigType) -> bool {
        self.bars.contains_key(symbol)
    }

    /// Gets the bars for the given symbol
    ///
    /// # Parameters
    ///
    /// * `symbol` - Symbol to retrieve bars for as a &str
    ///
    /// # Returns
    ///
    /// Result<Vec<DataBar>, &'static str> - Vector of bars if found, error string if not found
    pub fn get_bars(&self, symbol: &FileConfigType) -> Result<Vec<DataBar>, &'static str> {
        match self.bars.get(symbol) {
            Some(bars) => Ok(bars.to_vec()),
            None => Err("No data found for symbol"),
        }
    }
}
