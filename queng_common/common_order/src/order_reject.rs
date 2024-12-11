use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OrderRejectReason {
    #[default]
    OTHER = 0,
    UnknownSymbol = 1,
    UnknownExchange = 2,
    ExchangeClosed = 3,
    ExchangeUnreachable = 4,
    ExchangeResponseTimeout = 5,
    InvalidOrderType = 6,
    InvalidTimeInForce = 7,
    InvalidQuantity = 8,
    InvalidPrice = 9,
    InvalidStopPrice = 10,
    InvalidLossPrice = 11,
    InvalidOrderId = 12,
    OrderIdNotFound = 13,
    OrderStatusConflict = 14,
}

impl From<u8> for OrderRejectReason {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::OTHER,
            1 => Self::UnknownSymbol,
            2 => Self::UnknownExchange,
            3 => Self::ExchangeClosed,
            4 => Self::ExchangeUnreachable,
            5 => Self::ExchangeResponseTimeout,
            6 => Self::InvalidOrderType,
            7 => Self::InvalidTimeInForce,
            8 => Self::InvalidQuantity,
            9 => Self::InvalidPrice,
            10 => Self::InvalidStopPrice,
            11 => Self::InvalidLossPrice,
            12 => Self::InvalidOrderId,
            13 => Self::OrderIdNotFound,
            14 => Self::OrderStatusConflict,
            _ => Self::OTHER,
        }
    }
}

impl From<OrderRejectReason> for u8 {
    fn from(value: OrderRejectReason) -> Self {
        value as u8
    }
}

impl fmt::Display for OrderRejectReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
