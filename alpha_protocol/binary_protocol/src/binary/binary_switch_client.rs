use crate::{
    BinaryProtocol, FORWARD_MESSAGES_CODE, ForwardMessage, REGISTER_CLIENT_CODE, RegisterClient,
    UNREGISTER_CLIENT_CODE, UnRegisterClient, binary_utils,
};
use stream_errors::StreamError;

pub trait SwitchClient {
    fn register_client(&mut self, client_id: u16) -> Result<(), StreamError>;
    fn unregister_client(&mut self, client_id: u16) -> Result<(), StreamError>;
    fn send_message_to_client(
        &mut self,
        target_client_id: u16,
        payload: &[u8],
    ) -> Result<(), StreamError>;
}

impl<B: BinaryProtocol> SwitchClient for B {
    fn register_client(&mut self, client_id: u16) -> Result<(), StreamError> {
        let bytes =
            binary_utils::pack_raw_bytes_mut(REGISTER_CLIENT_CODE, &RegisterClient::new(client_id));

        self.try_send_raw_bytes_no_response(&bytes)?;

        Ok(())
    }
    fn unregister_client(&mut self, client_id: u16) -> Result<(), StreamError> {
        let bytes = binary_utils::pack_raw_bytes_mut(
            UNREGISTER_CLIENT_CODE,
            &UnRegisterClient::new(client_id),
        );

        self.try_send_raw_bytes_no_response(&bytes)?;

        Ok(())
    }

    fn send_message_to_client(
        &mut self,
        target_client_id: u16,
        payload: &[u8],
    ) -> Result<(), StreamError> {
        let bytes = binary_utils::pack_raw_bytes_mut_with_target(
            FORWARD_MESSAGES_CODE,
            target_client_id,
            &ForwardMessage::new(payload.to_vec()),
        );

        self.try_send_raw_bytes_no_response(&bytes)
    }
}
