use common_order::OrderCancel;
use sbe_bindings::{message_header_codec, Encoder, MessageType, OrderCancelEncoder, WriteBuf};
use sbe_types::SbeEncodeError;

/// Encodes an OrderCancel message into SBE (Simple Binary Encoding) format.
///
/// # Arguments
/// * `msg` - An OrderCancel struct containing order cancellation details
///
/// # Returns
/// * `Result<(usize, Vec<u8>), SbeEncodeError>` - A tuple containing:
///   * `usize`: The size limit of the encoded message
///   * `Vec<u8>`: The encoded message buffer as bytes
///   * Or `SbeEncodeError` if encoding fails
///
/// # Details
/// Encodes order details including exchange ID, client ID, client order ID,
/// and exchange order ID into a fixed-size binary format using SBE.
pub fn encode_order_cancel_message(msg: OrderCancel) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 47];

    let mut csg = OrderCancelEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(MessageType::OrderCancel);

    csg.exchange_id(msg.exchange_id() as u8);

    csg.client_id(msg.client_id());

    let mut byte_array = [0u8; 14];
    byte_array[..msg.client_order_id().len()].copy_from_slice(msg.client_order_id().as_bytes());

    csg.client_order_id(byte_array);

    let mut byte_array = [0u8; 20];
    byte_array[..msg.exchange_order_id().len()].copy_from_slice(msg.exchange_order_id().as_bytes());

    csg.exchange_order_id(byte_array);

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
