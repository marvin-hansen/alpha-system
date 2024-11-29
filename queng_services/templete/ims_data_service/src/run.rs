use crate::service::Server;
use common_errors::MessageProcessingError;
use futures_util::StreamExt;

impl Server {
    /// Starts the data service in an infinite loop.
    ///
    /// The service will consume messages from the control topic and handle them
    /// according to the service's configuration.
    ///
    /// # Errors
    ///
    /// The function will return an error if the service cannot be started.
    ///
    pub async fn run(self) -> Result<(), MessageProcessingError> {
        tokio::spawn(async move {
            let mut consumer_guard = self.consumer().write().await;
            let consumer = consumer_guard.consumer_mut();

            while let Some(message) = consumer.next().await {
                if let Ok(received_message) = message {
                    dbg!(
                    "Received event at offset: {}, current: {}, from stream: {stream}, topic: {topic}",
                    received_message.message.offset, received_message.current_offset
                );
                    self.handle_message(received_message.message)
                        .await
                        .expect("Failed to handle message");
                }
            }

            drop(consumer_guard)
        });

        Ok(())
    }
}
