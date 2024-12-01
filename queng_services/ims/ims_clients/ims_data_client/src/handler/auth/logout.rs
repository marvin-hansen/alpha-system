use crate::{ImsDataClient, ImsDataClientError};
use sbe_messages::ClientLogoutMessage;

impl ImsDataClient {
    pub async fn logout(&self) -> Result<(), ImsDataClientError> {
        // Construct login message
        let login_message = ClientLogoutMessage::new(self.client_id);

        // Encode SBE message
        let (_, message) = login_message
            .encode()
            .expect("[ImsDataClient/login]: Failed to encode message");

        // Send message out
        self.send_one_message(message)
            .await
            .expect("[ImsDataClient/login]: Failed to send login message!");

        Ok(())
    }
}
