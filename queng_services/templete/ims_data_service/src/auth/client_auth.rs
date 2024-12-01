use crate::service::Service;
use common_errors::MessageProcessingError;
use iggy::client::Client;

impl Service {
    /// Checks if a client with the specified ID is logged in.
    ///
    /// This method checks the client producers map to verify if a client with the given ID
    /// has an active producer, indicating they are logged in.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The unique identifier of the client to check
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// * `Ok(true)` if the client is logged in
    /// * `Ok(false)` if the client is not logged in
    ///
    /// # Errors
    ///
    /// Returns a `MessageProcessingError` if:
    /// * Failed to acquire read lock on client producers map
    /// * Lock is poisoned due to a panic in another thread
    pub(crate) async fn check_client_login(
        &self,
        client_id: u16,
    ) -> Result<bool, MessageProcessingError> {
        let client_db = self.client_producers().read().await;

        Ok(client_db.contains_key(&client_id))
    }

    /// Checks if a client is allowed to log in.
    ///
    /// This function should be overriden by the implementation of the service.
    /// The default implementation allows clients with an ID > 99.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to check
    ///
    /// # Returns
    ///
    /// `true` if the client is allowed, `false` otherwise
    pub(crate) fn client_allowed(&self, client_id: u16) -> bool {
        client_id >= 100
    }

    /// Logout a client by removing them from the client database.
    ///
    /// Locks the client manager, gets the config for the client,
    /// and attempts to remove them from the database.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to log out
    ///
    /// # Returns
    ///
    /// A Result with no value if the client was logged out successfully,
    /// or a MessageProcessingError if there was an issue.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there was an issue removing the client from the database.
    ///
    pub(crate) async fn client_logout(&self, client_id: u16) -> Result<(), MessageProcessingError> {
        // check if the client exists and is logged in
        let exists = self
            .check_client_login(client_id)
            .await
            .expect("Failed to check client login");
        if !exists {
            return Err(MessageProcessingError(format!(
                "Client with id {} not logged in",
                client_id
            )));
        }

        // lock the client_data_producers hashmap
        let mut client_data_producers = self.client_producers().write().await;

        // get the client data producer from the hashmap
        match client_data_producers.get(&client_id) {
            Some(message_stream) => {
                // shutdown the client of the client data producer
                match message_stream.iggy_client().shutdown().await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Failed to shutdown client: {e}");
                        return Err(MessageProcessingError(e.to_string()));
                    }
                }

                // Remove the client data producer from the hashmap
                client_data_producers
                    .remove(&client_id)
                    .expect("Failed to remove client from hashmap");
            }
            None => {
                return Err(MessageProcessingError(format!(
                    "Client with id {} not found",
                    client_id
                )))
            }
        };

        // Unlock the client_data_producers hashmap
        drop(client_data_producers);

        Ok(())
    }
}
