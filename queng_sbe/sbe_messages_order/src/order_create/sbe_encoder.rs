use common_order::OrderCreate;
use sbe_bindings::{
    binary_string_20_codec::BinaryString20Encoder, message_header_codec, message_type::MessageType,
    order_create_codec::OrderCreateEncoder, Encoder, WriteBuf,
};
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

    csg.client_order_id(msg.client_order_id().client_order_id_binary());

    let (first, second) = msg.symbol_id_exchange().exchange_order_id_binary();

    let mut symbol_id_encoder: BinaryString20Encoder<OrderCreateEncoder> =
        BinaryString20Encoder::default();
    //
    //
    symbol_id_encoder.first(first);
    symbol_id_encoder.second(second);

    symbol_id_encoder
        .parent()
        .expect("Failed to encode symbol id");

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
