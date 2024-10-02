use crate::error::ImsDataClientError;
use crate::ImsDataClient;
use common_data_bar::prelude::DataType;
use common_exchange::prelude::ExchangeID;

impl ImsDataClient {
    /// Sends a request to the IMS data client to start streaming data
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The exchange to get data from
    /// * `symbols` - The symbols to get data for
    /// * `data_type` - The type of data to get (trades, quotes, etc.)
    ///
    /// # Returns
    ///
    /// Returns the stream id if successful, error otherwise
    ///
    /// # Errors
    ///
    /// Can error if communication with the IMS data client fails
    ///
    pub async fn start_data(
        &mut self,
        _exchange_id: ExchangeID,
        _symbols: Vec<String>,
        _data_type: DataType,
    ) -> Result<u32, ImsDataClientError> {
        Err(ImsDataClientError(format!("{:?}", "Not implemented")))
    }

    /// Sends a request to the IMS data client to stop streaming data
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The exchange that the data is from
    /// * `stream_id` - The stream id to stop
    /// * `data_type` - The data type that was being streamed
    ///
    /// # Returns
    ///
    /// Returns Ok if successful, error otherwise
    ///
    /// # Errors
    ///
    /// Can error if communication with the IMS data client fails
    ///
    pub async fn stop_data(
        &mut self,
        _exchange_id: ExchangeID,
        _stream_id: u32,
        _data_type: DataType,
    ) -> Result<(), ImsDataClientError> {
        Err(ImsDataClientError(format!("{:?}", "Not implemented")))
    }

    /// Sends a request to the IMS data client to stop all streaming data
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The exchange to stop streaming data for
    ///
    /// # Returns
    ///
    /// Returns Ok if successful, error otherwise
    ///
    /// # Errors
    ///
    /// Can error if communication with the IMS data client fails
    ///
    pub async fn stop_all_data(
        &mut self,
        _exchange_id: ExchangeID,
    ) -> Result<(), ImsDataClientError> {
        Err(ImsDataClientError(format!("{:?}", "Not implemented")))
    }
}
