use common_data_bar::{OHLCVBar, TradeBar};
use common_errors::MessageProcessingError;
use message_shared::SendMessage;
use sbe_messages::{DataErrorType, SbeOHLCVBar, SbeTradeBar};

use crate::service::Service;
impl Service {
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
        match self.send_data(client_id, enc_trade_bar).await {
            Ok(()) => {}
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
        match self.send_data(client_id, enc_ohlcv_bar).await {
            Ok(()) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub(crate) async fn send_data(
        &self,
        client_id: u16,
        bytes: Vec<u8>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // lock the client_data_producers hashmap
        let client_data_producers = self.client_producers().read().await;

        // Get the producer for the client
        let producer = client_data_producers
            .get(&client_id)
            .expect("[QDGW/utils_message::send_client_data]: No producer found");

        // Send the messages to the client's topic/partition
        producer
            .iggy_producer()
            .send_one_message(bytes)
            .await
            .expect("Failed to send error");

        // Unlock the client_data_producers hashmap
        drop(client_data_producers);

        Ok(())
    }
}
