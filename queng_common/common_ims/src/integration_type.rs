use std::fmt::{Display, Formatter};

/// An ImsIntegrationType represents the type of integration to Interactive
/// Brokers.
///
/// # Variants
///
/// * `Data`: The data integration type.
/// * `Execution`: The execution integration type.
/// * `OMS`: The Order Management System integration type.
///
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ImsIntegrationType {
    Data = 0,
    Execution = 1,
    OMS = 2,
}

// Convert ImsIntegrationType to an u8
impl From<ImsIntegrationType> for u8 {
    #[inline]
    fn from(value: ImsIntegrationType) -> Self {
        value as u8
    }
}

// Convert ImsIntegrationType to an u32
impl From<ImsIntegrationType> for u32 {
    #[inline]
    fn from(value: ImsIntegrationType) -> Self {
        value as u32
    }
}

// Create ImsIntegrationType from an u8
impl From<u8> for ImsIntegrationType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => ImsIntegrationType::Data,
            1 => ImsIntegrationType::Execution,
            2 => ImsIntegrationType::OMS,
            _ => panic!("Invalid ImsIntegrationType value: {}", value),
        }
    }
}

// create ImsIntegrationType from an u16
impl From<u16> for ImsIntegrationType {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            0 => ImsIntegrationType::Data,
            1 => ImsIntegrationType::Execution,
            2 => ImsIntegrationType::OMS,
            _ => panic!("Invalid ImsIntegrationType value: {}", value),
        }
    }
}

// Create ImsIntegrationType from an u32
impl From<u32> for ImsIntegrationType {
    #[inline]
    fn from(value: u32) -> Self {
        match value {
            0 => ImsIntegrationType::Data,
            1 => ImsIntegrationType::Execution,
            2 => ImsIntegrationType::OMS,
            _ => panic!("Invalid ImsIntegrationType value: {}", value),
        }
    }
}

impl Display for ImsIntegrationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
