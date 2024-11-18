use common_data_bar::{OHLCVBar, TradeBar};
use common_errors::MessageProcessingError;
use sbe_messages::{DataErrorType, DataType, SbeOHLCVBar, SbeTradeBar};

use crate::service::Server;
impl Server {
    /// Sends a first bar message to the client to indicate the start of a data stream.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to send the message to.
    /// * `symbol_id` - The symbol id for the first bar message.
    /// * `data_type` - The data type (OHLCV or Trade) for encoding the first bar message.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error encoding or sending the first bar message.
    ///
    pub(crate) async fn send_first_bar(
        &self,
        client_id: u16,
        symbol_id: u16,
        data_type: &DataType,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Encode the first bar message
        let enc_first_bar = match sbe_utils::encode_first_bar(data_type, symbol_id) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // Send the first bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, enc_first_bar).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Sends a last bar message to the client to indicate the end of a data stream.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to send the message to.
    /// * `symbol_id` - The symbol id for the last bar message.
    /// * `data_type` - The data type (OHLCV or Trade) for encoding the last bar message.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error encoding or sending the last bar message.
    ///
    pub(crate) async fn send_last_bar(
        &self,
        client_id: u16,
        symbol_id: u16,
        data_type: &DataType,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Encode the last bar message
        let enc_last_bar = match sbe_utils::encode_last_bar(data_type, symbol_id).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        // Send the first bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, enc_last_bar).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    /// Sends a trade bar message to the client to inform it that the data stream starts.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to send the message to.
    /// * `bar` - The trade bar message to send.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error encoding or sending the first bar message.
    ///
    pub(crate) async fn send_trade_bar(
        &self,
        client_id: u16,
        bar: &TradeBar,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Encode the trade bar message
        let (_, enc_trade_bar) = SbeTradeBar::encode(bar.to_owned()).unwrap();

        // Send trade bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, enc_trade_bar).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }
    /// Sends an OHLCV bar message to the client to inform it that the data stream starts.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The id of the client to send the message to.
    /// * `symbol_id` - The symbol id for the first bar message.
    /// * `bar` - The OHLCV bar message to send.
    ///
    /// # Errors
    ///
    /// Returns a Result with the error variants:
    ///
    /// - `(DataErrorType, MessageProcessingError)` - Error encoding or sending the first bar message.
    ///
    pub(crate) async fn send_ohlcv_bar(
        &self,
        client_id: u16,
        bar: &OHLCVBar,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Encode the trade bar message
        let (_, enc_ohlcv_bar) = SbeOHLCVBar::encode(bar.to_owned()).unwrap();

        // Send the ohlcv bar message to inform the client that the data stream starts
        match self.send_client_data(client_id, enc_ohlcv_bar).await {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub(crate) async fn send_client_data(
        &self,
        _client_id: u16,
        _bytes: Vec<u8>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // // Lock the client_configs hashmap
        // let client_configs = self.client_configs().write().await;
        //
        // // Get the client config for the client
        // let iggy_config = client_configs.get(&client_id).unwrap();
        //
        // // lock the client_data_producers hashmap
        // let client_data_producers = self.client_producers().read().await;
        //
        // // Get the producer for the error channel
        // let producer = client_data_producers
        //     .get(&client_id)
        //     .expect("[QDGW/utils_message::send_client_data]: No producer found");

        // Send the messages to the client's topic/partition

        // Unlock the client_configs hashmap
        // drop(client_configs);

        // Unlock the client_data_producers hashmap
        // drop(client_data_producers);

        Ok(())
    }
}
