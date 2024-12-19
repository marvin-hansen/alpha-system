use common_order::OrderCreate;
use rust_decimal::prelude::ToPrimitive;
use sbe_bindings::{
    message_header_codec, message_type::MessageType, order_create_codec::OrderCreateEncoder,
    Encoder, WriteBuf,
};
use sbe_types::SbeEncodeError;

pub fn encode_order_create_message(msg: OrderCreate) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 75];

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

    // Self (csg) moves into symbol_id_encoder
    let mut symbol_id_encoder = csg.exchange_symbol_id_encoder();
    let (first, second) = msg.symbol_id_exchange().exchange_order_id_binary();
    symbol_id_encoder.first(first);
    symbol_id_encoder.second(second);

    // Move Self (csg) from symbol_id_encoder back into csg.
    csg = symbol_id_encoder
        .parent()
        .expect("Failed to encode symbol id");

    csg.order_type(msg.order_type().into());

    csg.order_side(msg.order_side().into());

    csg.time_in_force(msg.order_time_in_force().into());

    if let Some(time_expiry) = msg.time_expiry() {
        csg.time_expiry(time_expiry);
    }

    let mut qty_encoder = csg.order_qty_encoder();
    qty_encoder.num(
        msg.quantity()
            .mantissa()
            .to_i64()
            .expect("Failed to convert quantity decimal to i64"),
    );
    qty_encoder.scale(msg.quantity().scale() as u8);
    csg = qty_encoder.parent().expect("Failed to encode order qty");

    let mut price_encoder = csg.order_price_encoder();
    price_encoder.num(
        msg.price()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    price_encoder.scale(msg.price().scale() as u8);
    csg = price_encoder
        .parent()
        .expect("Failed to encode order price");

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
