use crate::order_id_client::ClientOrderID;
use crate::order_id_exchange::ExchangeOrderID;
use crate::{OrderExchangeSymbol, OrderSide, OrderType, TimeInForce};
use common_exchange::ExchangeID;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct OrderUpdate {
    exchange_id: ExchangeID,
    client_id: u16,
    client_order_id: ClientOrderID,
    exchange_order_id: ExchangeOrderID,
    symbol_id_exchange: OrderExchangeSymbol,
    order_type: OrderType,
    order_side: OrderSide,
    order_time_in_force: TimeInForce,
    // Expiration time. Conditionally required for orders with time_in_force = `GOOD_TILL_TIME_EXCHANGE`.
    time_expiry: Option<u64>,
    price: Decimal,
    quantity: Decimal,
}

impl OrderUpdate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        exchange_id: ExchangeID,
        client_id: u16,
        client_order_id: ClientOrderID,
        exchange_order_id: ExchangeOrderID,
        symbol_id_exchange: OrderExchangeSymbol,
        order_type: OrderType,
        order_side: OrderSide,
        order_time_in_force: TimeInForce,
        time_expiry: Option<u64>,
        price: Decimal,
        quantity: Decimal,
    ) -> Self {
        Self {
            exchange_id,
            client_id,
            client_order_id,
            exchange_order_id,
            symbol_id_exchange,
            order_type,
            order_side,
            order_time_in_force,
            time_expiry,
            price,
            quantity,
        }
    }
}

impl OrderUpdate {
    pub fn exchange_id(&self) -> ExchangeID {
        self.exchange_id
    }
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn client_order_id(&self) -> &ClientOrderID {
        &self.client_order_id
    }

    pub fn exchange_order_id(&self) -> &ExchangeOrderID {
        &self.exchange_order_id
    }

    pub fn symbol_id_exchange(&self) -> &OrderExchangeSymbol {
        &self.symbol_id_exchange
    }

    pub fn order_type(&self) -> &OrderType {
        &self.order_type
    }

    pub fn order_side(&self) -> &OrderSide {
        &self.order_side
    }

    pub fn order_time_in_force(&self) -> &TimeInForce {
        &self.order_time_in_force
    }

    pub fn time_expiry(&self) -> Option<u64> {
        self.time_expiry
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }
}

impl Display for OrderUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
