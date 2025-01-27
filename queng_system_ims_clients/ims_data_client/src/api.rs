use crate::{ImsDataClient, ImsDataClientError};
use sbe_types::DataType;
use trait_data_integration::EventProcessor;

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
    /// * `data_type_id` - `DataType` data type ID
    /// * `event_processor` - `EventProcessor` event processor
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn start_data(
        &self,
        symbol_id: String,
        data_type_id: DataType,
        event_processor: &(impl EventProcessor + Send + Sync + 'static),
    ) -> Result<(), ImsDataClientError> {
        self.client_start_data(symbol_id, data_type_id, event_processor)
            .await
    }

    /// Stops data for a given symbol and data type.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - String symbol ID
    /// * `data_type_id` - `DataType` data type ID
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn stop_data(
        &self,
        symbol_id: String,
        data_type_id: DataType,
    ) -> Result<(), ImsDataClientError> {
        self.client_stop_data(symbol_id, data_type_id).await
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
