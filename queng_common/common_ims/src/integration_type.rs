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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ImsIntegrationType {
    Data,
    Execution,
    OMS,
}

impl Display for ImsIntegrationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}
