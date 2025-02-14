/*
 * Copyright (c) "2025" . Marvin Hansen All Rights Reserved.
 */

use common_order::OrderUpdate;
use rust_decimal::prelude::ToPrimitive;
use sbe_bindings::{
    Encoder, WriteBuf, message_header_codec, message_type::MessageType,
    order_update_codec::OrderUpdateEncoder,
};
use sbe_types::SbeEncodeError;

pub fn encode_order_update_message(msg: OrderUpdate) -> Result<(usize, Vec<u8>), SbeEncodeError> {
    // precise buffer size
    let mut buffer = vec![0u8; 91];

    let mut csg = OrderUpdateEncoder::default();

    csg = csg.wrap(
        WriteBuf::new(buffer.as_mut_slice()),
        message_header_codec::ENCODED_LENGTH,
    );

    csg = csg.header(0).parent().expect("Failed to encode header");

    csg.message_type(MessageType::OrderUpdate);

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

    // Self (csg) moves
    let mut encoder = csg.exchange_order_id_encoder();
    let (first, second) = msg
        .exchange_order_id()
        .exchange_order_id_binary()
        .to_owned();
    encoder.first(first);
    encoder.second(second);

    // Move Self back into csg
    csg = encoder
        .parent()
        .expect("Failed to encode exchange order id");

    csg.order_type(msg.order_type().into());

    csg.order_side(msg.order_side().into());

    csg.time_in_force(msg.order_time_in_force().into());

    if let Some(time_expiry) = msg.time_expiry() {
        csg.time_expiry(time_expiry);
    }

    // Self (csg) moves
    let mut qty_encoder = csg.order_qty_encoder();
    qty_encoder.num(
        msg.quantity()
            .mantissa()
            .to_i64()
            .expect("Failed to convert quantity decimal to i64"),
    );
    qty_encoder.scale(msg.quantity().scale() as u8);
    // Move Self back into csg
    csg = qty_encoder.parent().expect("Failed to encode order qty");

    // Self (csg) moves
    let mut price_encoder = csg.order_price_encoder();
    price_encoder.num(
        msg.price()
            .mantissa()
            .to_i64()
            .expect("Failed to convert price decimal to i64"),
    );
    price_encoder.scale(msg.price().scale() as u8);
    // Move Self back into csg
    csg = price_encoder
        .parent()
        .expect("Failed to encode order price");

    let limit = csg.get_limit();
    Ok((limit, buffer))
}
