use crate::error::MDDBClientError;
use crate::MDDBClient;
use common_metadata::MetaExchange;

use proto_mddb_utils::prelude::*;

impl MDDBClient {
    /// Retrieves the total count of exchanges from the database.
    ///
    /// Returns a Result containing either the count as u64 or an MDDBClientError if the operation fails.
    ///
    pub async fn count_exchanges(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_count_exchanges_request();

        match client.count_exchanges(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Checks if an exchange ID exists in the database.
    ///
    /// # Arguments
    /// * `exchange_code` - The exchange code to check
    ///
    /// # Returns
    /// * `Result<bool, MDDBClientError>` - Ok(true) if exchange exists, Ok(false) if not, or an error
    ///
    pub async fn check_if_exchange_id_exists(
        &self,
        exchange_code: &str,
    ) -> Result<bool, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_check_if_exchange_exists_request(exchange_code);

        match client.check_if_exchange_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves exchange information from the client using the provided exchange code.
    ///
    /// # Arguments
    /// * `exchange_code` - A string slice containing the exchange identifier
    ///
    /// # Returns
    /// * `Result<MetaExchange, MDDBClientError>` - Exchange data on success, or error if not found/failed
    ///
    pub async fn get_exchange(&self, exchange_code: &str) -> Result<MetaExchange, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_exchange_request(exchange_code);

        match client.get_exchange(request).await {
            Ok(res) => {
                let exchange = res
                    .into_inner()
                    .exchange
                    .map(|exchange| proto_exchange_to_meta_exchange(&exchange))
                    .ok_or_else(|| MDDBClientError("Exchange not found".to_string()))?;
                Ok(exchange)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all available exchanges from the database.
    ///
    /// Returns a Result containing either a Vec of MetaExchange objects or an MDDBClientError.
    /// The exchanges are fetched via gRPC and converted from proto format to MetaExchange type.
    ///
    pub async fn get_all_exchanges(&self) -> Result<Vec<MetaExchange>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_exchanges_request();

        match client.get_all_exchanges(request).await {
            Ok(res) => {
                let exchanges = res
                    .into_inner()
                    .exchanges
                    .into_iter()
                    .map(|proto_exchange| proto_exchange_to_meta_exchange(&proto_exchange))
                    .collect();
                Ok(exchanges)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }
}
