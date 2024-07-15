use crate::SymbolManager;
use common_errors::prelude::LookupError;

impl SymbolManager {
    /// Looks up the name of an exchange by its ID.
    ///
    /// # Parameters
    ///
    /// * `exchange_id` - The ID of the exchange to look up.
    ///
    /// # Returns
    ///
    /// Returns the name of the exchange as a `Result<String, LookupError>`.
    ///
    ///
    /// # Errors
    ///
    /// Returns a `LookupError` if no exchange with the given ID exists.
    ///
    /// # Functionality
    ///
    /// Looks up the `exchange_id` key in the `index_to_exchange` map.
    /// If found, returns the exchange name.
    /// If not found, returns a `LookupError`.
    pub fn get_exchange_name(&self, exchange_id: u16) -> Result<String, LookupError> {
        let exchange_name = match self.index_to_exchange.get(&exchange_id) {
            Some(symbol) => symbol,
            None => {
                return Err(LookupError::new(format!(
                    "[SymbolManager]: Exchange not found for ID: {}",
                    exchange_id
                )));
            }
        };

        Ok(exchange_name.to_owned())
    }
}
