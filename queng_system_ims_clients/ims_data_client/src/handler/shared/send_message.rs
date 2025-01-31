/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use crate::{ImsDataClient, ImsDataClientError};
use trait_event_processor::*;

impl ImsDataClient {
    /// Sends a message to the control topic.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The message to send as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// If the message fails to send, it will return an `ImsDataClientError` with the error message.
    ///
    pub(crate) async fn send_one_message(&self, bytes: Vec<u8>) -> Result<(), ImsDataClientError> {
        match self.control_producer.process_one_event(bytes).await {
            Ok(_) => Ok(()),
            Err(e) => Err(ImsDataClientError(e.to_string())),
        }
    }
}
