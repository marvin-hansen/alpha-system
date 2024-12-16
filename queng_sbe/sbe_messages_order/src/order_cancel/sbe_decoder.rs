use common_exchange::ExchangeID;
use common_order::OrderCancel;
use sbe_bindings::order_cancel_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{MessageHeaderDecoder, OrderCancelDecoder, ReadBuf};
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
pub(crate) fn decode_order_cancel_message(buffer: &[u8]) -> Result<OrderCancel, SbeDecodeError> {
    let mut csg = OrderCancelDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let sbe_message_type = csg.message_type();
    let message_type = MessageType::from(sbe_message_type as u16);
    assert_eq!(message_type, MessageType::OrderCancel);

    let exchange_id = ExchangeID::from(csg.exchange_id());

    let client_id = csg.client_id();

    let raw_string = String::from_utf8(csg.client_order_id().to_vec()).expect("Invalid UTF-8");
    let client_order_id = raw_string.trim_matches(char::from(0)).to_string();

    let raw_string = String::from_utf8(csg.exchange_order_id().to_vec()).expect("Invalid UTF-8");
    let exchange_order_id = raw_string.trim_matches(char::from(0)).to_string();

    Ok(OrderCancel::new(
        exchange_id,
        client_id,
        client_order_id,
        exchange_order_id,
    ))
}
