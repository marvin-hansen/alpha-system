use crate::OrderSide;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionData {
    symbol_id: String,
    exchange_symbol_id: String,
    order_side: OrderSide,
    avg_entry_price: Decimal,
    quantity: Decimal,
    unrealized_pnl: Decimal,
    leverage: Decimal,
    cross_margin: Option<Decimal>,
    liquidation_price: Option<Decimal>,
    initial_margin: Option<Decimal>,
    maintenance_margin: Option<Decimal>,
    maintenance_margin_rate: Option<Decimal>,
    raw_data: Option<String>,
}

impl PositionData {
    pub fn new(
        symbol_id: String,
        exchange_symbol_id: String,
        order_side: OrderSide,
        avg_entry_price: Decimal,
        quantity: Decimal,
        unrealized_pnl: Decimal,
        leverage: Decimal,
        cross_margin: Option<Decimal>,
        liquidation_price: Option<Decimal>,
        initial_margin: Option<Decimal>,
        maintenance_margin: Option<Decimal>,
        maintenance_margin_rate: Option<Decimal>,
        raw_data: Option<String>,
    ) -> Self {
        Self {
            symbol_id,
            exchange_symbol_id,
            order_side,
            avg_entry_price,
            quantity,
            unrealized_pnl,
            leverage,
            cross_margin,
            liquidation_price,
            initial_margin,
            maintenance_margin,
            maintenance_margin_rate,
            raw_data,
        }
    }
}

impl PositionData {
    pub fn symbol_id(&self) -> &str {
        &self.symbol_id
    }

    pub fn exchange_symbol_id(&self) -> &str {
        &self.exchange_symbol_id
    }

    pub fn order_side(&self) -> &OrderSide {
        &self.order_side
    }

    pub fn avg_entry_price(&self) -> Decimal {
        self.avg_entry_price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn unrealized_pnl(&self) -> Decimal {
        self.unrealized_pnl
    }

    pub fn leverage(&self) -> Decimal {
        self.leverage
    }

    pub fn cross_margin(&self) -> Option<Decimal> {
        self.cross_margin
    }

    pub fn liquidation_price(&self) -> Option<Decimal> {
        self.liquidation_price
    }

    pub fn initial_margin(&self) -> Option<Decimal> {
        self.initial_margin
    }

    pub fn maintenance_margin(&self) -> Option<Decimal> {
        self.maintenance_margin
    }

    pub fn maintenance_margin_rate(&self) -> Option<Decimal> {
        self.maintenance_margin_rate
    }

    pub fn raw_data(&self) -> &Option<String> {
        &self.raw_data
    }
}

impl Display for PositionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
