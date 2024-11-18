use common_errors::MessageProcessingError;
use sbe_messages::{ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType};

use crate::service::Server;

impl Server {
    /// Sends a ClientError message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `client_error` - The ClientErrorType to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_client_error(
        &self,
        client_id: u16,
        client_error: ClientErrorType,
    ) -> Result<(), MessageProcessingError> {
        let message = ClientErrorMessage::new(client_id, client_error);
        let enc = message.encode();
        assert!(enc.is_ok());

        let (_, bytes) = enc.unwrap();

        // Send message
        self.send_error(bytes)
            .await
            .expect("Failed to send client error message");

        Ok(())
    }

    /// Sends a DataError message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `data_error` - The DataErrorType to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_data_error(
        &self,
        client_id: u16,
        data_error: DataErrorType,
    ) -> Result<(), MessageProcessingError> {
        let message = DataErrorMessage::new(client_id, data_error);
        let (_, bytes) = message
            .encode()
            .expect("Failed to encode data error message");

        // Send message
        self.send_error(bytes)
            .await
            .expect("Failed to send error message");

        Ok(())
    }

    pub(crate) async fn send_error(&self, _bytes: Vec<u8>) -> Result<(), MessageProcessingError> {
        // Send the error message

        Ok(())
    }
}
