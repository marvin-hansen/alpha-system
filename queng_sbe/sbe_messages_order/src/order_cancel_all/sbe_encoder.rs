use common_order::OrderCancelAll;
use sbe_bindings::{message_header_codec, Encoder, MessageType, OrderCancelAllEncoder, WriteBuf};
use sbe_types::SbeEncodeError;

pub(crate) fn encode_order_cancel_all_message(
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
