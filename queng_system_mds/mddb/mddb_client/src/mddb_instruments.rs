use crate::error::MDDBClientError;
use crate::MDDBClient;
use common_metadata::MetaInstrument;

use proto_mddb_utils::{
    get_all_instruments_for_base_asset_and_exchange_request,
    get_all_instruments_for_base_asset_request,
    get_all_instruments_for_base_quote_asset_and_exchange_request,
    get_all_instruments_for_exchange_request,
    get_all_instruments_for_quote_asset_and_exchange_request,
    get_all_instruments_for_quote_asset_request, get_all_instruments_request,
    get_check_if_instrument_exists_request, get_count_instruments_request,
    get_instrument_by_figi_request, get_instrument_by_id_request,
    get_instrument_by_pair_figi_request, get_lookup_instrument_exchange_pair_code_request,
    get_lookup_instrument_id_by_figi_request, get_lookup_instrument_id_by_pair_figi_request,
    proto_instrument_to_meta_instrument,
};

impl MDDBClient {
    /// Retrieves the total count of instruments from the database.
    ///
    /// Returns a Result containing either the count as u64 or an `MDDBClientError` if the operation fails.
    pub async fn count_instruments(&self) -> Result<u64, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_count_instruments_request();

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
        let request = get_check_if_instrument_exists_request(instrument_id);

        match client.check_if_instrument_id_exists(request).await {
            Ok(res) => Ok(res.get_ref().exists),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves a `MetaInstrument` by its instrument ID from the database.
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
    ) -> Result<Option<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_instrument_by_id_request(instrument_id);

        match client.get_instrument(request).await {
            Ok(res) => Ok(res
                .into_inner()
                .instrument
                .map(|instrument| proto_instrument_to_meta_instrument(&instrument))),
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
        let request = get_instrument_by_figi_request(instrument_figi);

        match client.get_instrument_by_figi(request).await {
            Ok(res) => Ok(res
                .into_inner()
                .instrument
                .map(|instrument| proto_instrument_to_meta_instrument(&instrument))),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    pub async fn get_instrument_by_pair_figi(
        &self,
        instrument_pair_figi: &str,
    ) -> Result<Option<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_instrument_by_pair_figi_request(instrument_pair_figi);

        match client.get_instrument_by_pair_figi(request).await {
            Ok(res) => Ok(res
                .into_inner()
                .instrument
                .map(|instrument| proto_instrument_to_meta_instrument(&instrument))),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Retrieves all available instruments from the market data database.
    ///
    /// Returns a Result containing either a Vector of `MetaInstrument` objects or an `MDDBClientError`.
    /// The function makes an async request to get all instruments and converts the proto responses
    /// into `MetaInstrument` format.
    ///
    pub async fn get_all_instruments(&self) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_instruments_request();

        match client.get_all_instruments(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
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
    /// * `Result<Vec<MetaInstrument>>` - A vector of `MetaInstrument` objects on success
    /// * `MDDBClientError` - Error if the retrieval fails
    ///
    pub async fn get_all_instruments_for_base_asset(
        &self,
        base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_instruments_for_base_asset_request(base_asset);

        match client.get_all_instruments_for_base_asset(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
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
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of `MetaInstrument` objects on success,
    ///   or `MDDBClientError` on failure
    ///
    pub async fn get_all_instruments_for_quote_asset(
        &self,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_instruments_for_quote_asset_request(quote_asset);

        match client.get_all_instruments_for_quote_asset(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
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
        let request = get_all_instruments_for_exchange_request(exchange_code);

        match client.get_all_instruments_for_exchange(request).await {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
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
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of `MetaInstrument` objects on success, or `MDDBClientError` on failure
    ///
    pub async fn get_all_instruments_for_base_asset_and_exchange(
        &self,
        exchange_code: &str,
        base_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request =
            get_all_instruments_for_base_asset_and_exchange_request(exchange_code, base_asset);

        match client
            .get_all_instruments_for_base_asset_and_exchange(request)
            .await
        {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
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
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of `MetaInstrument` on success, or `MDDBClientError` on failure
    ///
    pub async fn get_all_instruments_for_quote_asset_and_exchange(
        &self,
        exchange_code: &str,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request =
            get_all_instruments_for_quote_asset_and_exchange_request(exchange_code, quote_asset);

        match client
            .get_all_instruments_for_quote_asset_and_exchange(request)
            .await
        {
            Ok(res) => {
                let instruments = res
                    .into_inner()
                    .instruments
                    .into_iter()
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
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
    /// * `Result<Vec<MetaInstrument>, MDDBClientError>` - A vector of `MetaInstrument` objects on success, or `MDDBClientError` on failure
    ///
    pub async fn get_all_instruments_for_base_quote_asset_and_exchange(
        &self,
        exchange_code: &str,
        base_asset: &str,
        quote_asset: &str,
    ) -> Result<Vec<MetaInstrument>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_all_instruments_for_base_quote_asset_and_exchange_request(
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
                    .map(|proto_instrument| proto_instrument_to_meta_instrument(&proto_instrument))
                    .collect();
                Ok(instruments)
            }
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Looks up the instrument primary ID (key) for a exchange symbol pair code
    ///
    /// # Arguments
    /// * `instrument_exchange_pair_code` - The  exchange symbol pair code of the instrument to look up
    ///
    /// # Returns
    /// * `Result<String, MDDBClientError>` - The instrument primary ID (key) on success, or None if note found, or an error
    ///
    pub async fn lookup_instrument_id_by_exchange_pair_code(
        &self,
        instrument_exchange_pair_code: &str,
    ) -> Result<Option<String>, MDDBClientError> {
        let mut client = self.client.clone();
        let request =
            get_lookup_instrument_exchange_pair_code_request(instrument_exchange_pair_code);

        match client
            .lookup_instrument_id_by_exchange_pair_code(request)
            .await
        {
            Ok(res) => Ok(res.into_inner().instrument_id),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Looks up the instrument primary ID (key) for a given FIGI (Financial Instrument Global Identifier)
    ///
    /// # Arguments
    /// * `instrument_figi` - The FIGI of the instrument to look up
    ///
    /// # Returns
    /// * `Result<Option<String>, MDDBClientError>` - The instrument primary ID (key) if found, None if not found, or an error
    ///
    pub async fn lookup_instrument_id_by_figi(
        &self,
        instrument_figi: &str,
    ) -> Result<Option<String>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_lookup_instrument_id_by_figi_request(instrument_figi);

        match client.lookup_instrument_id_by_figi(request).await {
            Ok(res) => Ok(res.into_inner().instrument_id),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }

    /// Looks up the instrument primary ID (key) for a given Pair FIGI (Financial Instrument Global Identifier)
    ///
    /// # Arguments
    /// * `instrument_pair_figi` - The pair FIGI of the instrument to look up
    ///
    /// # Returns
    /// * `Result<String, MDDBClientError>` - The instrument primary ID (key) on success, or None if note found, or an error
    ///
    pub async fn lookup_instrument_id_by_pair_figi(
        &self,
        instrument_pair_figi: &str,
    ) -> Result<Option<String>, MDDBClientError> {
        let mut client = self.client.clone();
        let request = get_lookup_instrument_id_by_pair_figi_request(instrument_pair_figi);

        match client.lookup_instrument_id_by_pair_figi(request).await {
            Ok(res) => Ok(res.into_inner().instrument_id),
            Err(e) => Err(MDDBClientError(e.to_string())),
        }
    }
}
