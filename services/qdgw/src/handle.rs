use fluvio::dataplane::record::ConsumerRecord as Record;
use sbe_messages::prelude::{MessageType, StartDataMessage, StopAllDataMessage, StopDataMessage};

/// Handles an incoming record from the Fluvio stream.
///
/// # Parameters
///
/// * `record`: The incoming Fluvio consumer record to handle.
///
/// # Functionality
///
/// - Extracts the message value from the record and converts it to a byte buffer.
///
/// - Deserializes the message type from the buffer.
///
/// - Matches on the message type:
///   - `NullVal`: Logs receiving a null value message.
///   - `StartData`: Deserializes a `StartDataMessage` and calls `start_date`.
///   - `StopData`: Deserializes a `StopDataMessage` and calls `stop_date`.
///   - `StopAllData`: Deserializes a `StopAllDataMessage` and calls `stop_all_data`.
///
pub(crate) async fn handle_record(record: &Record) {
    let value = record.get_value().to_vec();
    let buffer = value.as_slice();
    let message_type = MessageType::from(buffer[2]);

    match message_type {
        MessageType::NullVal => {
            println!("[QDGW/handle::handle_record]: NullVal");
        }
        MessageType::StartData => {
            let start_data_msg = StartDataMessage::from(buffer);
            start_date(&start_data_msg).await;
        }
        MessageType::StopData => {
            let stop_data_msg = StopDataMessage::from(buffer);
            stop_date(&stop_data_msg).await;
        }
        MessageType::StopAllData => {
            let stop_all_data_msg = StopAllDataMessage::from(buffer);
            stop_all_data(&stop_all_data_msg).await;
        }
    }
}

async fn start_date(start_data_msg: &StartDataMessage) {
    println!(
        "[QDGW/handle::start_date]: start_data: {:?}",
        start_data_msg
    )
}

async fn stop_date(stop_data_msg: &StopDataMessage) {
    println!("[QDGW/handle::stop_date]: stop_data: {:?}", stop_data_msg)
}

async fn stop_all_data(stop_all_data_msg: &StopAllDataMessage) {
    println!(
        "[QDGW/handle::stop_all_data]: stop_all_data: {:?}",
        stop_all_data_msg
    )
}
