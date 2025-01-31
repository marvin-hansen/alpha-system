use crate::{ImsDataClient, ImsDataClientError};
use common_data_bar::TimeResolution;
use sbe_types::DataType;

impl ImsDataClient {
    /// Logs in the client via control channel.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn login(&self) -> Result<(), ImsDataClientError> {
        self.client_login().await
    }

    /// Logs out the client via control channel.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn logout(&self) -> Result<(), ImsDataClientError> {
        self.client_logout().await
    }

    /// Starts data for a given symbol and data type.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - String symbol ID
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn start_trade_data(&self, symbol_id: String) -> Result<(), ImsDataClientError> {
        self.client_start_trade_data(symbol_id).await
    }

    /// Starts OHLCV data for a given symbol, data type, and time resolution.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - String symbol ID
    /// * `time_resolution` - `TimeResolution` for the OHLCV data
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn start_ohlcv_data(
        &self,
        symbol_id: String,
        time_resolution: TimeResolution,
    ) -> Result<(), ImsDataClientError> {
        self.client_start_ohlcv_data(symbol_id, time_resolution)
            .await
    }

    /// Stops trade data for a given symbol
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - String symbol ID
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn stop_trade_data(&self, symbol_id: String) -> Result<(), ImsDataClientError> {
        self.client_stop_data(symbol_id, DataType::TradeData).await
    }

    /// Stops OHLCV data for a given symbol
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - String symbol ID
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn stop_ohlcv_data(&self, symbol_id: String) -> Result<(), ImsDataClientError> {
        self.client_stop_data(symbol_id, DataType::OHLCVData).await
    }

    /// Stops all data streams.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn stop_all_data(&self) -> Result<(), ImsDataClientError> {
        self.client_stop_all_data().await
    }
}
