/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_exchange::ExchangeID;
use common_order::{ClientOrderID, ExchangeOrderID, OrderCancel};
use sbe_bindings::order_cancel_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{
    message_header_codec::MessageHeaderDecoder, order_cancel_codec::OrderCancelDecoder, ReadBuf,
};
use sbe_types::{MessageType, SbeDecodeError};

/// Decodes an SBE-encoded OrderCancel message back into its struct form.
///
/// # Arguments
/// * `buffer` - A byte slice containing the encoded message
///
/// # Returns
/// * `Result<OrderCancel, SbeDecodeError>` - Either:
///   * The decoded OrderCancel struct containing order cancellation details
///   * Or `SbeDecodeError` if decoding fails
///
/// # Details
/// Validates message type, extracts and processes fields including exchange ID,
/// client ID, client order ID, and exchange order ID. Handles UTF-8 string
/// processing for order IDs.
pub fn decode_order_cancel_message(buffer: &[u8]) -> Result<OrderCancel, SbeDecodeError> {
    let mut csg = OrderCancelDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header, 0);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::OrderCancel);

    let exchange_id = ExchangeID::from(csg.exchange_id());

    let client_id = csg.client_id();

    let client_order_id = ClientOrderID::from(csg.client_order_id());

    let decoder = csg.exchange_order_id_decoder();
    let exchange_order_id = ExchangeOrderID::from((decoder.first(), decoder.second()));

    Ok(OrderCancel::new(
        exchange_id,
        client_id,
        client_order_id,
        exchange_order_id,
    ))
}
