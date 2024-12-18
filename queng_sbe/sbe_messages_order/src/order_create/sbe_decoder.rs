use common_exchange::ExchangeID;
use common_order::{ClientOrderID, OrderCreate, OrderSide, OrderType, TimeInForce};
use rust_decimal::Decimal;
use sbe_bindings::order_create_codec::SBE_TEMPLATE_ID;
use sbe_bindings::{MessageHeaderDecoder, OrderCreateDecoder, ReadBuf};
use sbe_types::SbeDecodeError;

pub(crate) fn decode_order_create_message(buffer: &[u8]) -> Result<OrderCreate, SbeDecodeError> {
    let mut csg = OrderCreateDecoder::default();
    let buf = ReadBuf::new(buffer);

    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    assert_eq!(SBE_TEMPLATE_ID, header.template_id());
    csg = csg.header(header);

    let exchange_id = ExchangeID::from(csg.exchange_id());

    let client_id = csg.client_id();

    let client_order_id = ClientOrderID::from(String::new()); // TODO

    let exchange_symbol_id = String::new(); // TODO
                                            //
    let order_type = OrderType::from(csg.order_type());

    let order_side = OrderSide::from(csg.order_side());

    let order_time_in_force = TimeInForce::from(csg.time_in_force());

    let time_expiry_decoder = csg.time_expiry_decoder();
    let time_expiry = if time_expiry_decoder.mantissa().is_some() {
        Some(time_expiry_decoder.mantissa().unwrap() as u64)
    } else {
        None
    };

    let qty_decoder = csg.order_qty_decoder();
    let quantity = Decimal::new(qty_decoder.mantissa(), qty_decoder.exponent() as u32);

    let price_decoder = csg.order_price_decoder();
    let price = Decimal::new(price_decoder.mantissa(), price_decoder.exponent() as u32);

    Ok(OrderCreate::new(
        exchange_id,
        client_id,
        client_order_id,
        exchange_symbol_id,
        order_type,
        order_side,
        order_time_in_force,
        time_expiry,
        quantity,
        price,
    ))
}
