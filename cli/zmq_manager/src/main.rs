use std::error::Error;
use std::time::Duration;

use common::prelude::{ExchangeID, SymbolID};
use fluvio::{Fluvio, Offset, RecordKey, TopicProducerConfigBuilder};
use sbe_messages::prelude::{MessageType, StartDataMessage, StopAllDataMessage, StopDataMessage};

// use crate::types::WorkflowOP;

// mod types;
// mod message_handle;

// const OP: WorkflowOP = WorkflowOP::TestData;

const TOPIC: &str = "echo";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send>> {
    // handle_workflow_op(&OP).await.expect("Failed to handle workflow op");

    let produce_handle = tokio::spawn(produce());
    let consume_handle = tokio::spawn(consume());

    match tokio::try_join!(produce_handle, consume_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                " Failed to start produce_handle and consume_handle: {:?}",
                e
            );
        }
    }

    Ok(())
}

async fn produce() -> Result<(), Box<dyn Error + Send>> {
    let fluvio = Fluvio::connect().await.unwrap();

    let config = TopicProducerConfigBuilder::default()
        .batch_size(250)
        .linger(Duration::from_micros(10))
        .build()
        .expect("Failed to create topic producer config");

    let producer = fluvio
        .topic_producer_with_config(TOPIC, config)
        .await
        .expect("Failed to create a producer");

    let message = StartDataMessage::new(ExchangeID::BinanceSpot, SymbolID::BTCUSD);
    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    producer
        .send(RecordKey::NULL, buffer)
        .await
        .expect("Failed to send Done!");

    producer.flush().await.expect("Failed to flush");

    let message = StopDataMessage::new(ExchangeID::BinanceSpot, SymbolID::BTCUSD);
    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    producer
        .send(RecordKey::NULL, buffer)
        .await
        .expect("Failed to send Done!");

    producer.flush().await.expect("Failed to flush");

    let message = StopAllDataMessage::new(ExchangeID::BinanceSpot);
    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 10);

    producer
        .send(RecordKey::NULL, buffer)
        .await
        .expect("Failed to send Done!");

    producer.flush().await.expect("Failed to flush");

    return Ok(())
}

/// Consumes events until a "Done!" event is read
async fn consume() -> Result<(), Box<dyn Error + Send>> {
    use futures::StreamExt;

    let consumer = fluvio::consumer(TOPIC, 0)
        .await
        .expect("Failed to create a consumer");

    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2]);
        println!("[zmq manager]: Message type: {}", message_type);

        if message_type == MessageType::StopAllData {
            break;
        }
    }

    Ok(())
}
