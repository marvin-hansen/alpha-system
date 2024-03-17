use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// An u8 encoded Enum that represents the unique ID of a service.
///
/// # Variants
///
/// * `NullVal`: Null value in case deserialization fails due to an unknown value.
/// * `Default`: Default value.
/// * `SMDB`: The SMDb service.
/// * `CMDB`: The CMDB service.
/// * `DBGW`: The DBGW service.
/// * `QDGW`: The QDGW service.
#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ServiceID {
    #[default]
    Default = 0x0_u8,
    SMDB = 0x1_u8,
    CMDB = 0x2_u8,
    DBGW = 0x3_u8,
    QDGW = 0x4_u8,
    SYMDB = 0x5_u8,
    VEX = 0x6_u8,
    ImsDataBinance = 0x7_u8,
}

impl ServiceID {
    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl From<i32> for ServiceID {
    /// Converts an `i32` value to a `ServiceID` variant.
    ///
    /// # Arguments
    ///
    /// * `value` - The `i32` value to convert
    ///
    /// # Returns
    ///
    /// The corresponding `ServiceID` variant for the `i32` value:
    ///
    /// * 0: `Default`
    /// * 1: `SMDB`
    /// * 2: `CMDB`
    /// * 3: `DBGW`
    /// * 4: `QDGW`
    /// * 5: `SYMDB`
    /// * 6: `VEX`
    ///
    /// If the value does not match a variant, returns `Default`.
    ///
    #[inline]
    fn from(v: i32) -> Self {
        match v {
            0x0_i32 => Self::Default,
            0x1_i32 => Self::SMDB,
            0x2_i32 => Self::CMDB,
            0x3_i32 => Self::DBGW,
            0x4_i32 => Self::QDGW,
            0x5_i32 => Self::SYMDB,
            0x6_i32 => Self::VEX,
            0x7_i32 => Self::ImsDataBinance,
            _ => Self::Default,
        }
    }
}

impl From<u8> for ServiceID {
    /// Converts a `u8` value to a `ServiceID` variant.
    ///
    /// # Arguments
    ///
    /// * `value` - The `u8` value to convert
    ///
    /// # Returns
    ///
    /// The corresponding `ServiceID` variant for the `u8` value:
    ///
    /// * 0: `Default`
    /// * 1: `SMDB`
    /// * 2: `CMDB`
    /// * 3: `DBGW`
    /// * 4: `QDGW`
    /// * 5: `SYMDB`
    /// * 6: `VEX`
    ///
    /// If the value does not match a variant, returns `Default`.
    ///
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::SMDB,
            2 => Self::CMDB,
            3 => Self::DBGW,
            4 => Self::QDGW,
            5 => Self::SYMDB,
            6 => Self::VEX,
            7 => Self::ImsDataBinance,
            _ => Self::Default,
        }
    }
}

impl ServiceID {
    pub fn from_string(n: &str) -> Option<ServiceID> {
        match n {
            "Default" => Some(ServiceID::Default),
            "SMDB" => Some(ServiceID::SMDB),
            "CMDB" => Some(ServiceID::CMDB),
            "DBGW" => Some(ServiceID::DBGW),
            "QDGW" => Some(ServiceID::QDGW),
            "SYMDB" => Some(ServiceID::SYMDB),
            "ImsDataBinance" => Some(ServiceID::ImsDataBinance),
            "VEX" => Some(ServiceID::VEX),
            _ => None,
        }
    }
}

impl Display for ServiceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
