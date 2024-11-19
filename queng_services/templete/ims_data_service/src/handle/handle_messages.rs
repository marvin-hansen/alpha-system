use crate::service::Server;
use common_errors::MessageProcessingError;
use sbe_messages::MessageType;

impl Server {
    /// Handles a single message by processing it and sending it to the appropriate
    /// manager for further processing.
    ///
    /// This method takes a message payload, processes it by calling the `process_message`
    /// method, and sends it to the appropriate handler for further processing.
    ///
    /// # Parameters
    ///
    /// * `self` - The Server instance
    /// * `message` - The message payload to be processed
    ///
    /// # Returns
    /// * Ok on success,
    /// * Err on any processing error
    ///
    pub(crate) async fn handle_message(
        &self,
        raw_message: &[u8],
    ) -> Result<(), MessageProcessingError> {
        //
        let message_type = MessageType::from(u16::from(raw_message[2]));

        match message_type {
            MessageType::ClientLogin => {
                todo!()
            }
            MessageType::ClientLogout => {
                todo!()
            }
            MessageType::StartData => {
                todo!()
            }
            MessageType::StopAllData => {
                todo!()
            }

            _ => Err(MessageProcessingError(
                "[handle::handle_message]: Unknown message type. Abort processing".to_string(),
            )),
        }
    }
}
