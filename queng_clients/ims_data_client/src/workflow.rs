use crate::error::ImsDataClientError;
use crate::{utils_proto, ImsDataClient};
use common::prelude::DataType;
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
        exchange_id: ExchangeID,
        symbols: Vec<String>,
        data_type: DataType,
    ) -> Result<u32, ImsDataClientError> {
        let request = utils_proto::get_start_data_request(exchange_id, symbols, data_type);

        let res = self.client.start_data(request).await;

        match res {
            Ok(response) => Ok(response.into_inner().stream_id),
            Err(e) => Err(ImsDataClientError(format!("{:?}", e))),
        }
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
        exchange_id: ExchangeID,
        stream_id: u32,
        data_type: DataType,
    ) -> Result<(), ImsDataClientError> {
        let request = utils_proto::get_stop_data_request(exchange_id, stream_id, data_type);

        let res = self.client.stop_data(request).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsDataClientError(format!("{:?}", e))),
        }
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
        exchange_id: ExchangeID,
    ) -> Result<(), ImsDataClientError> {
        let request = utils_proto::get_stop_all_data_request(exchange_id);

        let res = self.client.stop_all_data(request).await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsDataClientError(format!("{:?}", e))),
        }
    }
}
