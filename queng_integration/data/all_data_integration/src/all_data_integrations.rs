use crate::all_dispatch::DataIntegration;
use crate::binance_data_integration::BinanceDataIntegration;
use crate::mock_data_integration::MockDataIntegration;
use crate::vex_data_integration::VexDataIntegration;

/// Enum for specifying the type of data integration
///
/// Data integration is used for fetching data from an exchange. The supported
/// types of data integrations are Binance and Mock.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExchangeDataIntegration {
    BinanceData,
    MockData,
    VexData,
}

/// Returns a `DataIntegration` instance based on the provided `ExchangeDataIntegration` enum value.
///
/// This function takes an `ExchangeDataIntegration` enum value as input and uses a `match` statement
/// to create a new instance of either `MockDataIntegration` or `BinanceDataIntegration`. The created
/// instance is then converted to a `DataIntegration` using the `from` method.
///
/// # Arguments
///
/// * `exchange_data_integration`: The `ExchangeDataIntegration` enum value that determines which type of
///   `DataIntegration` instance to create.
///
/// # Returns
///
/// A `DataIntegration` instance corresponding to the provided `ExchangeDataIntegration` enum value.
///
pub fn get_data_integration(exchange_data_integration: ExchangeDataIntegration) -> DataIntegration {
    match exchange_data_integration {
        ExchangeDataIntegration::BinanceData => {
            DataIntegration::from(BinanceDataIntegration::new())
        }
        ExchangeDataIntegration::MockData => DataIntegration::from(MockDataIntegration::new()),
        ExchangeDataIntegration::VexData => DataIntegration::from(VexDataIntegration::new()),
    }
}
