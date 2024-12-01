use crate::service::Service;
use common_errors::MessageProcessingError;
use message_stream::MessageStream;
use sbe_messages::ClientErrorType;

impl Service {
    //
    /// Logs a client in by validating the client ID and creating a new message stream.
    ///
    /// Checks if the client is already logged in, and if not, checks if the client is allowed to log in.
    /// If the client is allowed to log in, a new message stream is created and the client is logged in.
    ///
    /// The hashmap of client data producers is locked while the client is logged in.
    ///
    /// # Parameters
    ///
    /// * `client_id`: The ID of the client to log in
    ///
    /// # Returns
    ///
    /// A `Result` with no value if the client was logged in successfully,
    /// or a `(ClientErrorType, MessageProcessingError)` if an error occurred.
    ///
    /// # Errors
    ///
    /// - `ClientAlreadyLoggedIn` if the client is already logged in
    /// - `ClientNotAuthorized` if the client is not allowed to log in
    /// - `ClientLogInError` if there was an issue creating the message stream or logging in the client
    pub(crate) async fn client_login(
        &self,
        client_id: u16,
    ) -> Result<(), (ClientErrorType, MessageProcessingError)> {
        self.dbg_print(&format!(
            "Checking if client with id {} is logged in",
            client_id
        ));
        let exists = match self.check_client_login(client_id).await {
            Ok(exists) => exists,
            Err(err) => {
                return Err((
                    ClientErrorType::ClientLogInError,
                    MessageProcessingError(format!(
                        "Failed to check if client with id {} is logged in due to error: { }",
                        client_id,
                        err.to_string()
                    )),
                ))
            }
        };

        if exists {
            self.dbg_print(&format!("Client with id {} already logged in", client_id));
            return Err((
                ClientErrorType::ClientAlreadyLoggedIn,
                MessageProcessingError(format!("Client with id {} already logged in", client_id)),
            ));
        }

        self.dbg_print(&format!(
            "Checking if client with id {} is allowed to log in",
            client_id
        ));

        let allowed = match self.check_client_login(client_id).await {
            Ok(allowed) => allowed,
            Err(err) => {
                return Err((
                    ClientErrorType::ClientLogInError,
                    MessageProcessingError(format!(
                    "Failed to check if client with id {} is allowed to log in due to error: { }",
                    client_id,
                    err.to_string()
                )),
                ))
            }
        };

        if !allowed {
            return Err((
                ClientErrorType::ClientNotAuthorized,
                MessageProcessingError(format!(
                    "Client with id {} not allowed to log in",
                    client_id
                )),
            ));
        }

        self.dbg_print(&format!(
            "Create a new message stream for client with id {}",
            client_id
        ));
        let message_stream = match MessageStream::new(client_id).await {
            Ok(stream) => stream,
            Err(err) => {
                return Err((
                    ClientErrorType::ClientLogInError,
                    MessageProcessingError(format!(
                        "Failed to create message stream for client with id {} due to error: { }",
                        client_id,
                        err.to_string()
                    )),
                ))
            }
        };

        // RW lock the client_data_producers hashmap
        let mut client_data_producers = self.client_producers().write().await;

        self.dbg_print(&format!("Login in client with id {}", client_id));
        match client_data_producers.insert(client_id, message_stream) {
            None => {
                return Err((
                    ClientErrorType::ClientLogInError,
                    MessageProcessingError(
                        format!("Failed to login client with id {}", client_id,),
                    ),
                ))
            }
            Some(_) => {}
        };

        // Unlock the client_data_producers hashmap
        drop(client_data_producers);

        self.dbg_print(&format!(
            "Client login successful for client with id {}",
            client_id
        ));

        Ok(())
    }
}
