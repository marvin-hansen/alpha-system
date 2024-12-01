use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_messages::ClientLoginMessage;

impl Service {
    /// Handles a client login message by validating the client ID and logging them in.
    ///
    /// Gets the client's control channel, checks if they are already logged in, and logs them in if not.
    /// Sends back any errors over the control channel.
    ///
    /// # Parameters
    ///
    /// - `client_login_msg`: The incoming ClientLoginMessage from the client
    ///
    /// # Returns
    ///
    /// Result with no value if successful, or a MessageProcessingError if an error occurs.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there is an issue getting the client's control channel, checking their login status,
    ///   or logging them in.
    ///
    /// ```
    pub(crate) async fn handle_client_login(
        &self,
        client_login_msg: &ClientLoginMessage,
    ) -> Result<(), MessageProcessingError> {
        // println!("::handle_client_login]: Extract the client ID from the message");
        let client_id = client_login_msg.client_id();

        match self.client_login(client_id).await {
            Ok(_) => {}
            Err((client_error_type, err)) => {
                println!(
                    "[handle_client_login] ClientLogInError: {:?}",
                    err.to_string()
                );

                match self.send_client_error(client_id, client_error_type).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!(
                            "[QDGW/handle_client_login] ClientLogInError: {:?}",
                            err.to_string()
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
