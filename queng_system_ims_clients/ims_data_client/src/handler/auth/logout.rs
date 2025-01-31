/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::{ImsDataClient, ImsDataClientError};
use sbe_messages_client::ClientLogoutMessage;

impl ImsDataClient {
    /// Logs out the client via control channel.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub(crate) async fn client_logout(&self) -> Result<(), ImsDataClientError> {
        self.dbg_print("logout");

        self.dbg_print("Construct logout message");
        let logout_message = ClientLogoutMessage::new(self.client_id);

        self.dbg_print("Encode SBE message");
        let (_, message) = logout_message
            .encode()
            .expect("[ImsDataClient/login]: Failed to encode message");

        self.dbg_print("Send message");
        self.send_one_message(message)
            .await
            .expect("[ImsDataClient/login]: Failed to send login message!");

        Ok(())
    }
}
