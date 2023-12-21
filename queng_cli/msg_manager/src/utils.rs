use common::prelude::MessageClientConfig;
use fluvio::{Fluvio, PartitionConsumer, TopicProducer};

pub fn get_client_config() -> MessageClientConfig {
    let id = 100;
    let name = "client-100".to_string();
    MessageClientConfig::new(id, name)
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
