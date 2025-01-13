use crate::ImsDataIntegrationError;

/// Trait to define an `EventProcessor` that can be used to process events
/// in a local context.
///
/// The `process` method is a callback that is called with the data fetched from
/// the exchange. The method takes a `&[Vec<u8>]` of data as input and returns a
/// `Result` of `()`.
#[allow(dead_code)] // Clippy can't see that the trait is used
#[trait_variant::make(EventProcessor: Send)]
pub trait LocalEventProcessor {
    /// Process a single byte message.
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), ImsDataIntegrationError>;

    /// Send a single byte message.
    ///
    /// The message is provided as a `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent.
    async fn send_one_message(&self, bytes: Vec<u8>) -> Result<(), ImsDataIntegrationError>;
    /// Send a batch of byte messages.
    ///
    /// The messages are provided as a `Vec` of `Vec<u8>`.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the messages cannot be sent.
    async fn send_batch_messages(
        &self,
        bytes_batch: &[Vec<u8>],
    ) -> Result<(), ImsDataIntegrationError>;
}

// Default implementation for `&T`
// https://users.rust-lang.org/t/hashmap-get-dereferenced/33558
impl<T: EventProcessor + Send + Sync> EventProcessor for &T {
    async fn process(&self, data: &[Vec<u8>]) -> Result<(), ImsDataIntegrationError> {
        (**self).process(data).await
    }

    async fn send_one_message(&self, bytes: Vec<u8>) -> Result<(), ImsDataIntegrationError> {
        (**self).send_one_message(bytes).await
    }

    async fn send_batch_messages(
        &self,
        bytes_batch: &[Vec<u8>],
    ) -> Result<(), ImsDataIntegrationError> {
        (**self).send_batch_messages(bytes_batch).await
    }
}
