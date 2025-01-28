use crate::{ImsDataClient, ImsDataClientError};
use futures_util::StreamExt;
use iggy::clients::consumer::IggyConsumer;
use trait_event_processor::EventProcessor;
impl ImsDataClient {
    pub(crate) async fn start_consume_data_messages(
        &self,
        consumer: &'static mut IggyConsumer,
        data_event_processor: &'static (impl EventProcessor + Sync),
    ) -> Result<(), ImsDataClientError> {
        let data_handler = tokio::spawn(async move {
            while let Some(received_message) = consumer.next().await {
                match received_message {
                    Ok(message) => data_event_processor
                        .process_one_event(message.message.payload.to_vec())
                        .await
                        .expect("[ImsDataClient]: Failed to process message"),
                    Err(e) => {
                        eprintln!(
                            "[ImsDataClient]:  Error polling messages from iggy message bus: {}",
                            e
                        );
                        break;
                    }
                }
            }
        });

        // Save the data handler to the client
        let mut guard = self.data_handler.write().await;
        *guard = Some(data_handler);

        Ok(())
    }
}
