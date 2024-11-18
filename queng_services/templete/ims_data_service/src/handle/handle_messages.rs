use crate::service::Server;
use common_errors::MessageProcessingError;
use sbe_messages::MessageType;

impl Server {
    pub(crate) async fn handle_message(
        &self,
        raw_message: &[u8],
    ) -> Result<(), MessageProcessingError> {
        //
        let message_type = MessageType::from(raw_message[2] as u16);

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
