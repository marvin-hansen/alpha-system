use crate::SbeEncodeError;
use common_data_bar::TradeBar;
use rust_decimal::prelude::ToPrimitive;

use sbe_bindings::{
    Encoder, MessageType as SbeMessageType, TradeBarEncoder, WriteBuf, ENCODED_LENGTH,
};

/// Encodes a `TradeBar` message to a byte buffer.
///
/// # Arguments
///
/// * `bar` - `TradeBar` to encode
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
/// - Create 28 byte buffer
/// - Create default `TradeBarEncoder`
/// - Wrap buffer in `WriteBuf`
/// - Encode header
/// - Encode `message_type`
/// - Encode `symbol_id`
/// - Encode `date_time` as timestamp
/// - Convert price to f32
/// - Encode price
/// - Convert volume to f32
/// - Encode volume
/// - Return encoded size and buffer
///
pub fn encode_trade_bar_message(bar: TradeBar) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    let mut buffer = vec![0u8; 34];

    let mut csg = TradeBarEncoder::default();

    csg = csg.wrap(WriteBuf::new(buffer.as_mut_slice()), ENCODED_LENGTH);

    csg = csg.header(0).parent().expect("Failed to encode header");

    let value = SbeMessageType::TradeBar;
    csg.message_type(value);

    let value = encoding_utils::str_to_int(bar.symbol_id()).expect("Failed to encode string");
    csg.symbol_id(value);

    let date_time = bar.date_time().timestamp_micros();
    csg.date_time(date_time);

    let price = bar
        .price()
        .to_f32()
        .expect("Failed to convert price to f32");
    csg.price(price);

    let volume = bar
        .volume()
        .to_f32()
        .expect("Failed to convert volume to u64");
    csg.volume(volume);

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
