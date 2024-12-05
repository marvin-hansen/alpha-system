use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_messages::StartDataMessage;

impl Service {
    /// Handle a start data message from a client. This involves verifying the message
    /// and then calling the start_data method of the service.
    ///
    /// # Errors
    ///
    /// If the start data message is invalid or the service is unable to start data for
    /// the client, a MessageProcessingError is returned.
    ///
    pub(crate) async fn handle_start_data(
        &self,
        start_data_message: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        self.dbg_print("handle_start_data");
        let client_id = *start_data_message.client_id();
        let exchange_id = start_data_message.exchange_id();
        let symbols = Vec::from(["BTCUSD".to_string()]);

        match self.start_data(client_id, exchange_id, &symbols).await {
            Ok(_) => {}
            Err((error_type, err)) => {
                // Print error
                println!("[handle_start_data]: StartDataError: {}", err);

                // Send error message to client
                match self.send_data_error(client_id, error_type).await {
                    Ok(_) => {}
                    Err(e) => {
                        println!("[handle_start_data]: SendDataError: {}", e);
                        return Err(e);
                    }
                }
            }
        }

        Ok(())
    }
}
