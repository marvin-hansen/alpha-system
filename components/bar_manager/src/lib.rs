use common::prelude::DataBar;
use std::collections::HashMap;

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
    pub fn add_bars(&mut self, symbol: &str, bars: Vec<DataBar>) {
        self.bars.insert(symbol.to_string(), bars);
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
    /// # Examples
    ///
    /// ```
    /// use common::prelude::DataBar;
    /// use bar_manager::BarManager;
    /// let mut bar_manager = BarManager::new();
    /// bar_manager.remove_bars("AAPL");
    /// ```
    pub fn remove_bars(&mut self, symbol: &str) {
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
    ///
    /// # Examples
    ///
    /// ```
    /// use common::prelude::DataBar;
    /// use bar_manager::BarManager;
    /// let mut bar_manager = BarManager::new();
    /// let has_data = bar_manager.has_data("AAPL");
    /// ```
    pub fn has_data(&self, symbol: &str) -> bool {
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
    ///
    /// # Examples
    ///
    /// ```
    /// use common::prelude::DataBar;
    /// use bar_manager::BarManager;    ///
    ///
    /// let symbol = "AAPL";
    /// let mut bar_manager = BarManager::new();
    ///  let bars = vec![/* test bars */];
    ///  bar_manager.add_bars(symbol, bars.clone());
    /// let bars = bar_manager.get_bars(symbol).expect("Failed to get data bars");
    /// ```
    pub fn get_bars(&self, symbol: &str) -> Result<Vec<DataBar>, &'static str> {
        match self.bars.get(symbol) {
            Some(bars) => Ok(bars.to_vec()),
            None => Err("No data found for symbol"),
        }
    }
}
