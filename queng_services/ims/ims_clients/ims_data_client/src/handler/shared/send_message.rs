use crate::{ImsDataClient, ImsDataClientError};
use message_shared::SendMessage;

impl ImsDataClient {
    /// Sends a message to the control topic.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The message to send as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub(crate) async fn send_one_message(&self, bytes: Vec<u8>) -> Result<(), ImsDataClientError> {
        match self.control_producer.send_one_message(bytes).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsDataClientError(e.to_string())),
        }
    }
}
