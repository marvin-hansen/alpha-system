use crate::error::MDDBClientError;
use crate::MDDBClient;
use common_metadata::prelude::{MetaAsset, MetaExchange, MetaInstrument};
use proto_mddb::proto::*;
use proto_mddb_utils::mddb_assets_utils as assets_utils;
use proto_mddb_utils::mddb_exchanges_utils as exchanges_utils;
use proto_mddb_utils::mddb_instruments_utils as instruments_utils;

impl MDDBClient {
    /// Returns the total number of assets in the MDDB
    ///
    /// # Returns
    /// * `Result<u64, MDDBClientError>` - The count of assets on success, or an error if the operation fails
    ///
    pub async fn count_assets(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_count_assets_request();

        match client.count_assets(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Checks if an asset with the given ID exists in the MDDB
    ///
    /// # Arguments
    /// * `asset_id` - The ID of the asset to check
    ///
    /// # Returns
    /// * `Result<bool, MDDBClientError>` - True if the asset exists, false otherwise, or an error if the operation fails
    ///
    pub async fn check_if_asset_id_exists(&self, asset_id: &str) -> Result<bool, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_check_if_asset_exists_request(asset_id);

        match client.check_if_asset_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves an asset from the MDDB by its ID
    ///
    /// # Arguments
    /// * `asset_id` - The ID of the asset to retrieve
    ///
    /// # Returns
    /// * `Result<Option<MetaAsset>, MDDBClientError>` - The asset if found, None if not found, or an error if the operation fails
    ///
    pub async fn get_asset(&self, asset_id: &str) -> Result<Option<MetaAsset>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_asset_request(asset_id);

        match client.get_asset(request).await {
            Ok(res) => Ok(res
                .into_inner()
                .asset
                .map(|asset| assets_utils::proto_asset_to_meta_asset(&asset))),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all assets from the database and converts them to MetaAsset format.
    ///
    /// # Returns
    /// * `Result<Vec<MetaAsset>, MDDBClientError>` - A vector of MetaAsset objects if successful,
    ///   or an MDDBClientError if the operation fails
    ///
    pub async fn get_all_assets(&self) -> Result<Vec<MetaAsset>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = assets_utils::get_all_assets_request();

        match client.get_all_assets(request).await {
            Ok(res) => {
                let assets = res
                    .into_inner()
                    .assets
                    .into_iter()
                    .map(|proto_asset: ProtoMetaAsset| {
                        assets_utils::proto_asset_to_meta_asset(&proto_asset)
                    })
                    .collect();

                Ok(assets)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves the total count of exchanges from the database.
    ///
    /// Returns a Result containing either the count as u64 or an MDDBClientError if the operation fails.
    ///
    pub async fn count_exchanges(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = exchanges_utils::get_count_exchanges_request();

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
        let request = exchanges_utils::get_check_if_exchange_exists_request(exchange_code);

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
        let request = exchanges_utils::get_exchange_request(exchange_code);

        match client.get_exchange(request).await {
            Ok(res) => {
                let exchange = res
                    .into_inner()
                    .exchange
                    .map(|exchange| exchanges_utils::proto_exchange_to_meta_exchange(&exchange))
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
        let request = exchanges_utils::get_all_exchanges_request();

        match client.get_all_exchanges(request).await {
            Ok(res) => {
                let exchanges = res
                    .into_inner()
                    .exchanges
                    .into_iter()
                    .map(|proto_exchange| {
                        exchanges_utils::proto_exchange_to_meta_exchange(&proto_exchange)
                    })
                    .collect();
                Ok(exchanges)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves the total count of instruments from the database.
    ///
    /// Returns a Result containing either the count as u64 or an MDDBClientError if the operation fails.
    pub async fn count_instruments(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_count_instruments_request();

        match client.count_instruments(request).await {
            Ok(res) => Ok(res.into_inner().count),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Checks if an instrument ID exists in the database.
    ///
    /// # Arguments
    /// * `instrument_id` - The ID of the instrument to check
    ///
    /// # Returns
    /// * `Result<bool, MDDBClientError>` - Ok(true) if instrument exists, Ok(false) if not, or Err on failure
    pub async fn check_if_instrument_id_exists(
        &self,
        instrument_id: &str,
    ) -> Result<bool, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_check_if_instrument_exists_request(instrument_id);

        match client.check_if_instrument_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves a MetaInstrument by its instrument ID from the database.
    ///
    /// # Arguments
    /// * `instrument_id` - The unique identifier of the instrument to retrieve
    ///
    /// # Returns
    /// * `Result<MetaInstrument, MDDBClientError>` - The instrument data if found, or an error if not found or on failure
    ///
    pub async fn get_instrument(
        &self,
        instrument_id: &str,
    ) -> Result<MetaInstrument, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_instrument_by_id_request(instrument_id);

        match client.get_instrument(request).await {
            Ok(res) => {
                let instrument = res
                    .into_inner()
                    .instrument
                    .ok_or_else(|| MDDBClientError("Instrument not found".to_string()))?;
                Ok(instruments_utils::proto_instrument_to_meta_instrument(
                    &instrument,
                ))
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves an instrument by its FIGI identifier from the market data database.
    ///
    /// # Arguments
    /// * `instrument_figi` - The FIGI identifier string of the instrument to retrieve
    ///
    /// # Returns
    /// * `Result<Option<MetaInstrument>, MDDBClientError>` - The instrument data if found, or error if request fails
    ///
    pub async fn get_instrument_by_figi(
        &self,
        instrument_figi: &str,
    ) -> Result<Option<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_instrument_by_figi_request(instrument_figi);

        match client.get_instrument_by_figi(request).await {
            Ok(res) => Ok(res.into_inner().instrument.map(|instrument| {
                instruments_utils::proto_instrument_to_meta_instrument(&instrument)
            })),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all available instruments from the market data database.
    ///
    /// Returns a Result containing either a Vector of MetaInstrument objects or an MDDBClientError.
    /// The function makes an async request to get all instruments and converts the proto responses
    /// into MetaInstrument format.
    ///
    pub async fn get_all_instruments(&self) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_all_instruments_request();

        match client.get_all_instruments(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all trading instruments for a specified base asset.
    ///
    /// # Arguments
    /// * `base_asset` - The base asset identifier as a string
    ///
    /// # Returns
    /// * `Result<Vec<MetaInstrument>>` - A vector of MetaInstrument objects on success
    /// * `MDDBClientError` - Error if the retrieval fails
    ///
    pub async fn get_all_instruments_for_base_asset(
        &self,
        base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_all_instruments_for_base_asset_request(base_asset);

        match client.get_all_instruments_for_base_asset(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all trading instruments for a specified quote asset.
    ///
    /// # Arguments
    /// * `quote_asset` - The quote asset identifier as a string
    ///
    /// # Returns
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of MetaInstrument objects on success,
    ///   or MDDBClientError on failure
    ///
    pub async fn get_all_instruments_for_quote_asset(
        &self,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_all_instruments_for_quote_asset_request(quote_asset);

        match client.get_all_instruments_for_quote_asset(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all instruments for a specified exchange.
    ///
    /// # Arguments
    /// * `exchange_code` - The code identifier for the exchange
    ///
    /// # Returns
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of instruments on success, or an error
    ///
    pub async fn get_all_instruments_for_exchange(
        &self,
        exchange_code: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_all_instruments_for_exchange_request(exchange_code);

        match client.get_all_instruments_for_exchange(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all trading instruments for a specific base asset and exchange.
    ///
    /// # Arguments
    /// * `exchange_code` - The code identifier for the exchange
    /// * `base_asset` - The base asset symbol
    ///
    /// # Returns
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of MetaInstrument objects on success, or MDDBClientError on failure
    ///
    pub async fn get_all_instruments_for_base_asset_and_exchange(
        &self,
        exchange_code: &str,
        base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_all_instruments_for_base_asset_and_exchange_request(
            exchange_code,
            base_asset,
        );

        match client
            .get_all_instruments_for_base_asset_and_exchange(request)
            .await
        {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all instruments for a specific exchange and quote asset.
    ///
    /// # Arguments
    /// * `exchange_code` - The code identifying the exchange
    /// * `quote_asset` - The quote asset to filter instruments by
    ///
    /// # Returns
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of MetaInstrument on success, or MDDBClientError on failure
    ///
    pub async fn get_all_instruments_for_quote_asset_and_exchange(
        &self,
        exchange_code: &str,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_all_instruments_for_quote_asset_and_exchange_request(
            exchange_code,
            quote_asset,
        );

        match client
            .get_all_instruments_for_quote_asset_and_exchange(request)
            .await
        {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all instruments for a specific exchange and asset pair.
    ///
    /// # Arguments
    /// * `exchange_code` - The code identifier for the exchange
    /// * `base_asset` - The base asset symbol
    /// * `quote_asset` - The quote asset symbol
    ///
    /// # Returns
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of MetaInstrument objects on success, or MDDBClientError on failure
    ///
    pub async fn get_all_instruments_for_base_quote_asset_and_exchange(
        &self,
        exchange_code: &str,
        base_asset: &str,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request =
            instruments_utils::get_all_instruments_for_base_quote_asset_and_exchange_request(
                exchange_code,
                base_asset,
                quote_asset,
            );

        match client
            .get_all_instruments_for_base_quote_asset_and_exchange(request)
            .await
        {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| {
                        instruments_utils::proto_instrument_to_meta_instrument(&proto_instrument)
                    })
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Looks up the exchange pair code name for a given instrument ID.
    ///
    /// # Arguments
    /// * `instrument_id` - The ID of the instrument to look up
    ///
    /// # Returns
    /// * `Result<String, MDDBClientError>` - The exchange pair code name on success, or an error if lookup fails
    ///
    pub async fn lookup_instrument_exchange_pair_code_name(
        &self,
        instrument_id: &str,
    ) -> Result<String, MDDBClientError> {
        let mut client = self.client.clone();
        let request =
            instruments_utils::get_lookup_instrument_exchange_pair_code_request(instrument_id);

        match client.lookup_instrument_exchange_pair_code(request).await {
            Ok(res) => Ok(res.into_inner().instrument_exchange_pair_code),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Looks up the FIGI (Financial Instrument Global Identifier) for a given instrument ID.
    ///
    /// # Arguments
    /// * `instrument_id` - The ID of the instrument to look up
    ///
    /// # Returns
    /// * `Result<Option<String>, MDDBClientError>` - The FIGI if found, None if not found, or an error
    ///
    pub async fn lookup_instrument_figi(
        &self,
        instrument_id: &str,
    ) -> Result<Option<String>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = instruments_utils::get_lookup_instrument_figi_request(instrument_id);

        match client.lookup_instrument_figi(request).await {
            Ok(res) => Ok(res.into_inner().instrument_pair_figi),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }
}
