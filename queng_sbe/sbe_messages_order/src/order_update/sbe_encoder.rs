use common_order::OrderUpdate;
use sbe_bindings::{
    message_header_codec, message_type::MessageType, order_update_codec::OrderUpdateEncoder,
    WriteBuf,
};
use sbe_types::SbeEncodeError;

pub fn encode_order_update_message(msg: OrderUpdate) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 100];

    let mut csg = OrderUpdateEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(MessageType::OrderUpdate);

    csg.exchange_id(msg.exchange_id() as u8);

    csg.client_id(msg.client_id());

    Ok((0, vec![]))
}
