use std::fmt;

/// The status of an order
#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum OrderStatus {
    /// The order is being routed to its destination
    ROUTING = 1,
    /// The order has been routed to its destination
    ROUTED = 2,
    /// The order has been received by its destination
    RECEIVED = 3,
    /// The order is pending new
    PendingNew = 4,
    /// The order is new
    NEW = 5,
    /// The order is partially filled
    PartiallyFilled = 6,
    /// The order is filled
    Filled = 7,
    /// The order is pending cancel
    PendingCancel = 8,
    /// The order is canceled
    Canceled = 9,
    /// The order is pending replace
    PendingReplace = 10,
    /// The order is replaced
    Replaced = 11,
    /// The order is rejected
    Rejected = 12,
}

impl From<u8> for OrderStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ROUTING,
            2 => Self::ROUTED,
            3 => Self::RECEIVED,
            4 => Self::PendingNew,
            5 => Self::NEW,
            6 => Self::PartiallyFilled,
            7 => Self::Filled,
            8 => Self::PendingCancel,
            9 => Self::Canceled,
            10 => Self::PendingReplace,
            11 => Self::Replaced,
            12 => Self::Rejected,
            _ => Self::ROUTING,
        }
    }
}

impl From<OrderStatus> for u8 {
    fn from(value: OrderStatus) -> Self {
        value as u8
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
