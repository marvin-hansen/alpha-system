use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OrderSide {
    Buy = 0x1_u8,
    Sell = 0x2_u8,
}

impl From<OrderSide> for u8 {
    fn from(side: OrderSide) -> Self {
        side as u8
    }
}

impl From<u8> for OrderSide {
    fn from(value: u8) -> Self {
        match value {
            0x1_u8 => Self::Buy,
            0x2_u8 => Self::Sell,
            _ => Self::Buy,
        }
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
