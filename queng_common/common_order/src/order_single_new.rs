use crate::{OrderSide, OrderType, TimeInForce};
use common_exchange::ExchangeID;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderSingleNew {
    exchange_id: ExchangeID,
    client_order_id: String,
    symbol_id: String,
    symbol_id_exchange: String,
    order_type: OrderType,
    order_side: OrderSide,
    time_in_force: TimeInForce,
    // Expiration time. Conditionally required for orders with time_in_force = `GOOD_TILL_TIME_EXCHANGE`.
    time_expiry: Option<u64>,
    price: Decimal,
    quantity: Decimal,
}

impl OrderSingleNew {
    pub fn new(
        exchange_id: ExchangeID,
        client_order_id: String,
        symbol_id: String,
        symbol_id_exchange: String,
        order_type: OrderType,
        order_side: OrderSide,
        time_in_force: TimeInForce,
        time_expiry: Option<u64>,
        quantity: Decimal,
        price: Decimal,
    ) -> Self {
        Self {
            exchange_id,
            client_order_id,
            symbol_id,
            symbol_id_exchange,
            order_type,
            order_side,
            time_in_force,
            time_expiry,
            price,
            quantity,
        }
    }
}

impl OrderSingleNew {
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }

    pub fn client_order_id(&self) -> &str {
        &self.client_order_id
    }

    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }

    pub fn symbol_id_exchange(&self) -> &str {
        &self.symbol_id_exchange
    }

    pub fn order_type(&self) -> &OrderType {
        &self.order_type
    }

    pub fn order_side(&self) -> &OrderSide {
        &self.order_side
    }

    pub fn time_in_force(&self) -> &TimeInForce {
        &self.time_in_force
    }

    pub fn time_expiry(&self) -> Option<u64> {
        self.time_expiry
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn price(&self) -> Decimal {
        self.price
    }
}

impl Display for OrderSingleNew {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
