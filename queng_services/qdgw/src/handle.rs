use fluvio::dataplane::record::ConsumerRecord;
use common::errors::MessageProcessingError;
use sbe_messages::prelude::{ClientLoginMessage, ClientLogoutMessage, MessageType, StartDataMessage, StopAllDataMessage, StopDataMessage};
use crate::service::Server;

impl Server {
    /// Handles an incoming record from the Fluvio stream.
    ///
    /// # Parameters
    /// * `record`: The incoming Fluvio consumer record to handle.
    ///
    /// # Functionality
    /// - Extracts the message value from the record and converts it to a byte buffer.
    /// - Deserializes the message type from the buffer.
    /// - Matches on the message type:
    ///   - `NullVal`: Logs receiving a null value message.
    ///   - `StartData`: Deserializes a `StartDataMessage` and calls `start_date`.
    ///   - `StopData`: Deserializes a `StopDataMessage` and calls `stop_date`.
    ///   - `StopAllData`: Deserializes a `StopAllDataMessage` and calls `stop_all_data`.
    pub(crate) async fn handle_record(
        &self,
        record: &ConsumerRecord,
    ) -> Result<(), MessageProcessingError> {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2]);

        match message_type {
            MessageType::UnknownMessageType => Err(MessageProcessingError(
                "[QDGW/handle::handle_record]:  Fluvio consumer record contained a null value"
                    .to_string(),
            )),

            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(buffer);
                self.client_login(&client_login_msg).await
            }

            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(buffer);
                self.client_logout(&client_logout_msg).await
            }

            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(buffer);
                self.start_date(&start_data_msg).await
            }
            MessageType::StopData => {
                let stop_data_msg = StopDataMessage::from(buffer);
                self.stop_date(&stop_data_msg).await
            }
            MessageType::StopAllData => {
                let stop_all_data_msg = StopAllDataMessage::from(buffer);
                self.stop_all_data(&stop_all_data_msg).await
            }
        }
    }
}
