use common::prelude::MessageClientConfig;
use fluvio::{Fluvio, PartitionConsumer, TopicProducer};

pub fn get_client_config() -> MessageClientConfig {
    MessageClientConfig::new(100)
}

pub async fn get_producer(topic: &str) -> TopicProducer {
    let fluvio = Fluvio::connect().await.unwrap();

    let producer = fluvio
        .topic_producer(topic)
        .await
        .expect("Failed to create a producer");

    producer
}

pub async fn get_consumer(topic: &str) -> PartitionConsumer {
    let consumer = fluvio::consumer(topic, 0)
        .await
        .expect("Failed to create a consumer");

    consumer
}
