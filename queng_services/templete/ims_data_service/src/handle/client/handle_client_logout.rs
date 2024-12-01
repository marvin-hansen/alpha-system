use crate::service::Service;
use common_errors::MessageProcessingError;
use sbe_messages::{ClientErrorType, ClientLogoutMessage};

impl Service {
    /// Handles a client logout message by validating the client ID and logging them out.
    ///
    /// Gets the client's control channel, checks if they are logged in, and logs them out if so.
    /// Sends back any errors over the control channel.
    ///
    /// # Parameters
    ///
    /// - `client_logout_msg`: The incoming ClientLogoutMessage from the client
    ///
    /// # Returns
    ///
    /// Result with no value if successful, or a MessageProcessingError if an error occurs.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there is an issue getting the client's control channel, checking their login status,
    ///   or logging them out.
    ///
    pub(crate) async fn handle_client_logout(
        &self,
        client_logout_msg: &ClientLogoutMessage,
    ) -> Result<(), MessageProcessingError> {
        // println!("[QDGW/handle_client::client_logout]");

        // println!("::handle_client_logout]: Extract the client ID from the message");
        let client_id = client_logout_msg.client_id();

        // println!("::handle_client_logout]: Check if the client is logged in");
        let exists = self.check_client_login(client_id).await;

        match exists {
            Ok(exists) => match exists {
                true => {
                    // println!("[::handle_client_logout]: Client is logged in, proceed with logout");
                    let res = self.client_logout(client_id).await;
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            // Print error
                            println!(
                                "[QDGW/handle_client_logout]: ClientLogOutError: {:?}",
                                err.to_string()
                            );

                            // return an ClientLogOutError to the client
                            let client_error_type = ClientErrorType::ClientLogOutError;
                            match self.send_client_error(client_id, client_error_type).await {
                                Ok(_) => {}
                                Err(err) => println!(
                                    "[QDGW/handle_client_logout]: ClientLogInError: {:?}",
                                    err.to_string()
                                ),
                            }
                        }
                    }
                }
                // client does not exist, return an ClientNotLoggedIn error to the client
                false => {
                    // println!("[::handle_client_logout]: Client is not logged in, return an ClientNotLoggedIn error to the client");
                    let client_error_type = ClientErrorType::ClientNotLoggedIn;
                    match self.send_client_error(client_id, client_error_type).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!(
                                "[QDGW/handle_client_logout]: ClientAlreadyLoggedIn: {:?}",
                                err
                            );
                        }
                    }
                }
            },
            // Something went horribly wrong, log the message, and return an unknown error
            Err(err) => {
                println!(
                    "[QDGW/handle_client_logout] UnknownClientError: {:?}",
                    err.to_string()
                );

                let client_error_type = ClientErrorType::UnknownClientError;
                match self.send_client_error(client_id, client_error_type).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!(
                            "[QDGW/handle_client_logout]: UnknownClientError: {:?}",
                            err.to_string()
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
