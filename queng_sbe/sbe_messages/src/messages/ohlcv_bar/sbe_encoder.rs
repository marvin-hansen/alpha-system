use crate::SbeEncodeError;
use common_data_bar::OHLCVBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    message_header_codec, DataBarEncoder, Encoder, MessageType as SbeMessageType, WriteBuf,
};

/// Encodes an `OHLCVBar` to a byte buffer.
///
/// # Arguments
///
/// * `bar` - `OHLCVBar` to encode
///
/// # Returns
///
/// (usize, `Vec<u8>`) - Tuple containing encoded size and byte buffer
///
/// # Errors
///
/// Returns Err if encoding fails
///
/// # Process
///
/// - Create 58 byte buffer
/// - Create default `DataBarEncoder`
/// - Wrap buffer in `WriteBuf`
/// - Encode header
/// - Encode `message_type`
/// - Encode `symbol_id`
/// - Encode `date_time`
/// - Encode and convert `open_price` to f32
/// - Encode and convert `high_price` to f32
/// - Encode and convert `low_price` to f32
/// - Encode and convert `close_price` to f32
/// - Encode and convert volume to f32
/// - Return encoded size and buffer
///
pub fn encode_data_bar_message(bar: OHLCVBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size is 58 bytes for the entire message.
    let mut buffer = vec![0u8; 58];

    let mut csg = DataBarEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(SbeMessageType::DataBar);

    // Convert string symbol id into fixed sized char [u8; 20]
    let mut byte_array = [0u8; 20];
    byte_array[..bar.symbol_id().len()].copy_from_slice(bar.symbol_id().as_bytes());
    csg.symbol_id(byte_array);

    csg.date_time(bar.date_time().timestamp_micros());

    let open_price = bar
        .open()
        .to_f32()
        .expect("Failed to convert open price to f32");
    csg.open_price(open_price);

    let high_price = bar
        .high()
        .to_f32()
        .expect("Failed to convert high price to f32");
    csg.high_price(high_price);

    let low_price = bar
        .low()
        .to_f32()
        .expect("Failed to convert low price to f32");
    csg.low_price(low_price);

    let close_price = bar
        .close()
        .to_f32()
        .expect("Failed to convert close price to f32");
    csg.close_price(close_price);

    let volume = bar
        .volume()
        .to_f32()
        .expect("Failed to convert volume to u64");
    csg.volume(volume);

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
