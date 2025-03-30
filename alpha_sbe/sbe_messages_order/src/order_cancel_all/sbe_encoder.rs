/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::OrderCancelAll;
use sbe_bindings::{
    Encoder, WriteBuf, message_header_codec, message_type::MessageType,
    order_cancel_all_codec::OrderCancelAllEncoder,
};
use sbe_types::SbeEncodeError;

/// Encodes an OrderCancelAll message into SBE (Simple Binary Encoding) format.
///
/// # Arguments
/// * `msg` - An OrderCancelAll struct containing details for cancelling all orders
///
/// # Returns
/// * `Result<(usize, Vec<u8>), SbeEncodeError>` - A tuple containing:
///   * `usize`: The size limit of the encoded message
///   * `Vec<u8>`: The encoded message buffer as bytes
///   * Or `SbeEncodeError` if encoding fails
///
/// # Details
/// Encodes basic order cancellation details including exchange ID and client ID
/// into a fixed-size binary format using SBE.
pub fn encode_order_cancel_all_message(
    msg: OrderCancelAll,
) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 13];

    let mut csg = OrderCancelAllEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(MessageType::OrderCancelAll);

    csg.exchange_id(msg.exchange_id() as u8);

    csg.client_id(msg.client_id());

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
