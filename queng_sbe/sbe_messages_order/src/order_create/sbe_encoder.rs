use common_order::OrderCreate;
use sbe_bindings::{message_header_codec, Encoder, MessageType, OrderCreateEncoder, WriteBuf};
use sbe_types::SbeEncodeError;

pub fn encode_order_create_message(msg: OrderCreate) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 100];

    let mut csg = OrderCreateEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(MessageType::OrderCreate);

    csg.exchange_id(msg.exchange_id() as u8);

    csg.client_id(msg.client_id());

    // Convert string symbol id into fixed sized char [u8; 14]
    let mut byte_array = [0u8; 14];
    byte_array[..msg.client_order_id().len()].copy_from_slice(msg.client_order_id().as_bytes());
    csg.client_order_id(byte_array);

    // Convert string symbol id into fixed sized char [u8; 20]
    let mut byte_array = [0u8; 20];
    byte_array[..msg.symbol_id_exchange().len()]
        .copy_from_slice(msg.symbol_id_exchange().as_bytes());
    csg.exchange_symbol_id(byte_array);

    csg.order_type(msg.order_type().into());

    csg.order_side(msg.order_side().into());

    csg.time_in_force(msg.order_time_in_force().into());

    // if let Some(time_expiry) = msg.time_expiry() {
    //     csg.timeExpiry(time_expiry);
    //     // csg.time_expiry_encoder().e
    // }

    // csg.order_price_encoder()

    // let mut price = csg.order_price_encoder();
    // price.mantissa()

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
