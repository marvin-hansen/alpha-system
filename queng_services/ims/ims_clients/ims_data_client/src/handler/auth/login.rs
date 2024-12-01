use crate::{ImsDataClient, ImsDataClientError};
use sbe_messages::ClientLoginMessage;

impl ImsDataClient {
    /// Logs in the client via control channel.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub async fn login(&self) -> Result<(), ImsDataClientError> {
        // Construct login message
        let login_message = ClientLoginMessage::new(self.client_id);

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
